use crate::api;
use api::Api;
use log::debug;
use reqwest::{RequestBuilder, Response, StatusCode};
use serde::de::DeserializeOwned;
use std::collections::HashMap;
use std::future::Future;

/// Configuration for the API http client
#[derive(Clone, Debug, Default)]
pub struct Client {
    /// Base URL for the bitbucket server. It must end with `/rest`.
    pub base_path: String,
    /// The HTTP client to use for making requests.
    pub http_client: reqwest::Client,
    /// The API token to use for authentication.
    pub api_token: String,
}

/// The bitbucket API client
impl Client {
    /// Bitbucket's `api` API.
    pub fn api(self) -> Api {
        Api { client: self }
    }

    // TODO add other APIs here as needed e.g. /default-reviewers, etc
}

/// Create a new bitbucket API client
///
/// # Arguments
/// * `base_path` - The base URL for the bitbucket server. It must end with `/rest`.
/// * `api_token` - The API token to use for authentication.
///
/// # Returns
/// A new bitbucket API client
///
/// # Examples
///
/// Basic example of creating a new client and calling an API.
///
/// ```no_run
/// use bitbucket_server_rs::client::{new, ApiError, ApiRequest, ApiResponse};
/// use bitbucket_server_rs::api::build_status_get::BuildStatus;
///
/// async fn example() -> ApiResponse<BuildStatus> {
///     let client = new(
///         "https://bitbucket-server/rest",
///         "API_TOKEN"
///     );
///
///     client
///        .api()
///        .build_status_get(
///            "PROJECT_KEY",
///            "COMMIT_ID",
///            "REPOSITORY_SLUG",
///        )
///        .key("ABC123")
///        .build()
///        .unwrap()
///        .send()
///        .await
/// }
/// ```
pub fn new(base_path: &str, api_token: &str) -> Client {
    Client {
        base_path: base_path.to_string(),
        http_client: reqwest::Client::new(),
        api_token: api_token.to_string(),
    }
}

/// Implementations for the bitbucket API client
impl Client {
    /// Create a request builder
    pub async fn builder(&self, req: RequestBuilder) -> RequestBuilder {
        req.header("Authorization", format!("Bearer {}", self.api_token))
            .header("Content-Type", "application/json")
    }

    /// Set an HTTP Client with a custom configuration
    ///
    ///
    pub fn with_http_client(&mut self, http_client: reqwest::Client) {
        self.http_client = http_client;
    }

    /// Send a GET request
    pub async fn get<T: ApiRequest>(
        &self,
        uri: &str,
        params: Option<HashMap<String, String>>,
    ) -> ApiResponse<T::Output> {
        let uri = format!("{}/{}", self.base_path, uri);
        let get = self.http_client.get(uri).query(&params);

        let req = self
            .builder(get)
            .await
            .build()
            .expect("Failed to build request");

        let response = self.http_client.execute(req).await.map_err(|e| {
            debug!("Error sending request: {:?}", e);
            ApiError::RequestError
        })?;

        Self::process_response::<T>(response).await
    }

    pub async fn post<T: ApiRequest>(
        &self,
        uri: &str,
        body: &str,
    ) -> ApiResponse<<T as ApiRequest>::Output> {
        let uri = format!("{}/{}", self.base_path, uri);
        let post = self.http_client.post(uri).body(body.to_string());

        let req = self
            .builder(post)
            .await
            .build()
            .expect("Failed to build request");

        let response = self.http_client.execute(req).await.map_err(|e| {
            debug!("Error sending request: {:?}", e);
            ApiError::RequestError
        })?;

        Self::process_response::<T>(response).await
    }

    async fn process_response<T: ApiRequest>(
        response: Response,
    ) -> ApiResponse<<T as ApiRequest>::Output> {
        match response.status() {
            status if status.is_success() => {
                let json = response.text().await.map_err(|e| {
                    debug!("Error reading response: {:?}", e);
                    ApiError::ResponseError
                })?;

                Self::make_api_response::<T>(json.as_str())
            }
            status if status == StatusCode::UNAUTHORIZED || status == StatusCode::FORBIDDEN => {
                Err(ApiError::Unauthorized)
            }
            status if status.is_client_error() => Err(ApiError::HttpClientError(
                status.as_u16(),
                response.text().await.unwrap_or_default(),
            )),
            status if status.is_server_error() => Err(ApiError::HttpServerError(
                status.as_u16(),
                response.text().await.unwrap_or_default(),
            )),
            _ => Err(ApiError::UnexpectedResponse(
                response.status().as_u16(),
                format!(
                    "Unexpected Response [{}]: {}",
                    response.status(),
                    response.text().await.unwrap_or_default()
                ),
            )),
        }
    }

    fn make_api_response<T: ApiRequest>(json: &str) -> ApiResponse<<T as ApiRequest>::Output> {
        // if the response is empty, Ok(None) means the response was successful but empty
        if json.len() == 0 {
            return Ok(None);
        }

        // deserialize into the request's output type
        let data = serde_json::from_str::<T::Output>(json)
            .map_err(|e| ApiError::DeserializationError(e.to_string()))?;

        Ok(Some(data))
    }
}

/// The response from the API
/// This is a `Result` type that contains an `Option` of the `ApiObject` or an `ApiError`
///
pub type ApiResponse<T> = Result<Option<T>, ApiError>;

/// This trait is used to implement the API requests
pub trait ApiRequest {
    /// The type of the response to deserialize to
    type Output: DeserializeOwned;

    /// Build the request and send
    fn send(&self) -> impl Future<Output = ApiResponse<Self::Output>> + Send;
}

/// The error type received from the API
#[derive(Debug)]
pub enum ApiError {
    /// Error building the request
    RequestError,

    /// Error getting the response
    ResponseError,

    /// Authentication error
    Unauthorized,

    /// HTTP 4xx errors
    HttpClientError(u16, String),

    /// HTTP 5xx errors
    HttpServerError(u16, String),

    /// Unexpected response
    UnexpectedResponse(u16, String),

    /// Error deserializing the response body
    DeserializationError(String),
}
