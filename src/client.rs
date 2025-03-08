//! # Bitbucket Server REST API Client
//!
//! This module provides the core client functionality for interacting with the Bitbucket Server REST API.
//! It includes the HTTP client, request/response handling, error types, and utility functions.

use crate::api;
use api::Api;
use log::debug;
use reqwest::{RequestBuilder, Response, StatusCode};
use serde::de::DeserializeOwned;
use std::collections::HashMap;
use std::future::Future;

/// Configuration for the Bitbucket Server API HTTP client.
///
/// This struct holds all the necessary configuration for making API requests to a Bitbucket Server instance.
#[derive(Clone, Default, Debug)]
pub struct Client {
    /// Base URL for the bitbucket server. It must end with `/rest`.
    pub base_path: String,
    
    /// The HTTP client to use for making requests.
    pub http_client: reqwest::Client,
    
    /// The API token to use for authentication.
    pub api_token: String,
}

/// The Bitbucket API client implementation.
impl Client {
    /// Access Bitbucket's `api` API endpoints.
    ///
    /// This method returns an `Api` struct that provides access to all the API endpoints
    /// under the `/rest/api` path.
    ///
    /// # Returns
    ///
    /// An `Api` struct that can be used to access API endpoints.
    pub fn api(self) -> Api {
        Api { client: self }
    }

    // TODO add other APIs here as needed e.g. /default-reviewers, etc
}

/// Create a new Bitbucket API client.
///
/// This function creates a new client with the specified base path and API token.
///
/// # Arguments
///
/// * `base_path` - The base URL for the Bitbucket server. It must end with `/rest`.
/// * `api_token` - The API token to use for authentication.
///
/// # Returns
///
/// A new Bitbucket API client.
///
/// # Examples
///
/// Basic example of creating a new client and calling an API:
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

/// HTTP request and response handling implementations for the Bitbucket API client.
impl Client {
    /// Create a request builder with authentication headers.
    ///
    /// This method adds the necessary authentication and content type headers to a request.
    ///
    /// # Arguments
    ///
    /// * `req` - The request builder to add headers to.
    ///
    /// # Returns
    ///
    /// A request builder with the headers added.
    pub async fn builder(&self, req: RequestBuilder) -> RequestBuilder {
        req.header("Authorization", format!("Bearer {}", self.api_token))
            .header("Content-Type", "application/json")
    }

    /// Set a custom HTTP client with specific configuration.
    ///
    /// This method allows you to use a custom HTTP client with specific configuration
    /// options, such as timeouts, proxies, etc.
    ///
    /// # Arguments
    ///
    /// * `http_client` - The custom HTTP client to use.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use bitbucket_server_rs::client::new;
    /// use reqwest::ClientBuilder;
    /// use std::time::Duration;
    ///
    /// let mut client = new("https://bitbucket-server/rest", "API_TOKEN");
    ///
    /// // Create a custom HTTP client with a 30-second timeout
    /// let http_client = ClientBuilder::new()
    ///     .timeout(Duration::from_secs(30))
    ///     .build()
    ///     .expect("Failed to build HTTP client");
    ///
    /// // Set the custom HTTP client
    /// client.with_http_client(http_client);
    /// ```
    pub fn with_http_client(&mut self, http_client: reqwest::Client) {
        self.http_client = http_client;
    }

    /// Send a GET request to the Bitbucket Server API.
    ///
    /// This method sends a GET request to the specified URI with the given query parameters.
    ///
    /// # Arguments
    ///
    /// * `uri` - The URI to send the request to, relative to the base path.
    /// * `params` - Optional query parameters to include in the request.
    ///
    /// # Returns
    ///
    /// A Result containing either the response data or an error.
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

    /// Send a POST request to the Bitbucket Server API.
    ///
    /// This method sends a POST request to the specified URI with the given body.
    ///
    /// # Arguments
    ///
    /// * `uri` - The URI to send the request to, relative to the base path.
    /// * `body` - The body to include in the request.
    ///
    /// # Returns
    ///
    /// A Result containing either the response data or an error.
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

    /// Process the response from the Bitbucket Server API.
    ///
    /// This method processes the response from the API, handling different status codes
    /// and converting the response body to the expected output type.
    ///
    /// # Arguments
    ///
    /// * `response` - The response from the API.
    ///
    /// # Returns
    ///
    /// A Result containing either the response data or an error.
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

    /// Convert a JSON string to an API response.
    ///
    /// This method converts a JSON string to an API response, handling empty responses
    /// and deserialization errors.
    ///
    /// # Arguments
    ///
    /// * `json` - The JSON string to convert.
    ///
    /// # Returns
    ///
    /// A Result containing either the deserialized data or an error.
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

/// The response from the API.
///
/// This is a `Result` type that contains an `Option` of the response data or an `ApiError`.
/// The `Option` is used because some API responses may be empty (e.g., successful DELETE requests).
pub type ApiResponse<T> = Result<Option<T>, ApiError>;

/// Trait for implementing API requests.
///
/// This trait defines the interface for all API requests. It requires implementing
/// the `Output` associated type and the `send` method.
pub trait ApiRequest {
    /// The type of the response to deserialize to.
    type Output: DeserializeOwned;

    /// Build the request and send it to the API.
    ///
    /// # Returns
    ///
    /// A Future that resolves to an ApiResponse containing either the response data or an error.
    fn send(&self) -> impl Future<Output = ApiResponse<Self::Output>> + Send;
}

/// Error types that can occur when making API requests.
///
/// This enum represents the different types of errors that can occur when making
/// API requests to the Bitbucket Server API.
#[derive(Debug)]
pub enum ApiError {
    /// Error building the request.
    RequestError,

    /// Error getting the response.
    ResponseError,

    /// Authentication error (HTTP 401 or 403).
    Unauthorized,

    /// HTTP 4xx client errors.
    ///
    /// Contains the status code and response body.
    HttpClientError(u16, String),

    /// HTTP 5xx server errors.
    ///
    /// Contains the status code and response body.
    HttpServerError(u16, String),

    /// Unexpected response.
    ///
    /// Contains the status code and response body.
    UnexpectedResponse(u16, String),

    /// Error deserializing the response body.
    ///
    /// Contains the error message.
    DeserializationError(String),
}
