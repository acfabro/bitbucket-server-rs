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

impl Error {
    /// Check if the error is an authentication error (HTTP 401 or 403).
    ///
    /// # Returns
    ///
    /// `true` if the error is an authentication error, `false` otherwise.
    ///
    /// # Example
    ///
    /// ```
    /// use bitbucket_server_rs::Error;
    ///
    /// let error = Error::Unauthorized;
    /// assert!(error.is_unauthorized());
    /// ```
    pub fn is_unauthorized(&self) -> bool {
        matches!(self, Error::Unauthorized)
    }

    /// Check if the error is a request error.
    ///
    /// # Returns
    ///
    /// `true` if the error is a request error, `false` otherwise.
    ///
    /// # Example
    ///
    /// ```
    /// use bitbucket_server_rs::Error;
    ///
    /// let error = Error::RequestError("Failed to build request".to_string());
    /// assert!(error.is_request_error());
    /// ```
    pub fn is_request_error(&self) -> bool {
        matches!(self, Error::RequestError(_))
    }

    /// Check if the error is a response error.
    ///
    /// # Returns
    ///
    /// `true` if the error is a response error, `false` otherwise.
    ///
    /// # Example
    ///
    /// ```
    /// use bitbucket_server_rs::Error;
    ///
    /// let error = Error::ResponseError("Failed to parse response".to_string());
    /// assert!(error.is_response_error());
    /// ```
    pub fn is_response_error(&self) -> bool {
        matches!(self, Error::ResponseError(_))
    }

    /// Check if the error is an unexpected error.
    ///
    /// # Returns
    ///
    /// `true` if the error is an unexpected error, `false` otherwise.
    ///
    /// # Example
    ///
    /// ```
    /// use bitbucket_server_rs::Error;
    ///
    /// let error = Error::Unexpected("Something went wrong".to_string());
    /// assert!(error.is_unexpected());
    /// ```
    pub fn is_unexpected(&self) -> bool {
        matches!(self, Error::Unexpected(_))
    }
}