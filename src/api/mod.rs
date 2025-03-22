//! # Bitbucket Server API
//!
//! This module contains implementations for various Bitbucket Server REST API endpoints.
//! Each submodule represents a specific API endpoint or group of related endpoints.
//!
//! ## API Structure
//!
//! The API is organized into the following modules:
//!
//! - `build_status`: Common types and utilities for build status operations
//! - `build_status_get`: API for retrieving build status information
//! - `build_status_post`: API for posting build status updates
//! - `pull_request_changes_get`: API for retrieving pull request changes
//! - `pull_request_post`: API for creating pull requests
//!
//! ## Usage Pattern
//!
//! All API endpoints follow a similar pattern:
//!
//! 1. Start with a client instance
//! 2. Call the appropriate API method
//! 3. Set any required parameters
//! 4. Build the request
//! 5. Send the request
//! 6. Process the response
//!
//! For example:
//!
//! ```no_run
//! use bitbucket_server_rs::{new, ApiRequest};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = new("https://bitbucket-server/rest", "API_TOKEN");
//!
//!     let response = client
//!         .api()
//!         .build_status_get("PROJECT", "COMMIT", "REPO")
//!         .key("build-123")
//!         .build()?
//!         .send()
//!         .await?;
//!
//!     Ok(())
//! }
//! ```

use crate::client::Client;

pub mod build_status;
pub mod build_status_get;
pub mod build_status_post;
pub mod pull_request_changes_get;
pub mod pull_request_post;

// Note: We intentionally avoid re-exporting types from submodules here
// to prevent potential namespace collisions. Users should import types
// directly from their specific submodules.

/// Bitbucket's `api` API. i.e. `https://bitbucket-server/rest/api`
///
/// This struct serves as the entry point for all API operations.
/// It holds a reference to the HTTP client that will be used for making requests.
pub struct Api {
    /// The http client to use for making requests. This includes
    /// the base URL, the HTTP client, and the API token.
    pub client: Client,
}
