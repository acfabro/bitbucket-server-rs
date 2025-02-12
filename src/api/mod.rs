use crate::client::Client;

mod build_status_get;
mod build_status_post;
mod pull_request_changes_get;

/// Bitbucket's `api` API. i.e. `https://bibucket-server/rest/api`
pub struct Api {
    /// The http client to use for making requests. This includes
    /// the base URL, the HTTP client, and the API token.
    pub client: Client,
}
