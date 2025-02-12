use crate::api;

/// Configuration for the API http client
#[derive(Clone, Debug)]
pub struct Client {
    /// Base URL for the bitbucket server. It must end with `/rest`.
    pub base_path: String,
    /// The HTTP client to use for making requests.
    pub client: reqwest::Client,
    /// The API token to use for authentication.
    pub api_token: String,
}

impl Client {
    /// Bitbucket's `api` API.
    pub fn api(self) -> api::Api {
        api::Api { client: self }
    }
}

/// Create a new bitbucket API client
///
/// # Arguments
/// * `base_path` - The base URL for the bitbucket server. It must end with `/rest`.
/// * `client` - The HTTP client to use for making requests.
/// * `api_token` - The API token to use for authentication.
///
/// # Returns
/// A new bitbucket API client
///
/// # Example
/// ```rust
/// use bitbucket_server_rs::client;
///
/// async fn example() -> Result<(), String> {
///     let client = client::new(
///         "https://bitbucket-server/rest".to_string(),
///          reqwest::Client::new(),
///         "API_TOKEN".to_string()
///     );
///
///     let _ = client.api()
///         .get_pull_request_changes(
///             "GOLF".to_string(),
///             "115".to_string(),
///             "golf-course".to_string(),
///             "src/".to_string()
///         )
///         .send()
///         .await?;
///
///     Ok(())
/// }
/// ```
pub fn new(base_path: String, client: reqwest::Client, api_token: String) -> Client {
    Client {
        base_path,
        client,
        api_token,
    }
}
