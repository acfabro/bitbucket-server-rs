use thiserror::Error;

/// Error types that can occur when making API requests.
///
/// This enum represents the different types of errors that can occur when making
/// API requests to the Bitbucket Server API.
#[derive(Debug, Error)]
pub enum Error {
    /// Error building the request.
    #[error("Error building the request: {0}")]
    RequestError(String),

    /// Error getting the response.
    #[error("Error getting the response: {0}")]
    ResponseError(String),

    /// Authentication error (HTTP 401 or 403).
    #[error("Authentication error")]
    Unauthorized,

    /// Unexpected error with a custom message.
    #[error("Unexpected error: {0}" )]
    Unexpected(String),
}