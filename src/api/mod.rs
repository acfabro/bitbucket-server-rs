use crate::client::Client;

pub mod build_status;
pub mod build_status_get;
pub mod build_status_post;
pub mod pull_request_changes_get;

/// Bitbucket's `api` API. i.e. `https://bibucket-server/rest/api`
pub struct Api {
    /// The http client to use for making requests. This includes
    /// the base URL, the HTTP client, and the API token.
    pub client: Client,
}
