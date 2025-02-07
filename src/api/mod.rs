use crate::client::Client;

pub mod build_status;
pub mod pull_request_changes;

/// Bitbucket's `api` API. i.e. `https://bibucket-server/rest/api`
pub struct Api<'a> {
    /// The http client to use for making requests. This includes
    /// the base URL, the HTTP client, and the API token.
    pub client: &'a Client,
}
