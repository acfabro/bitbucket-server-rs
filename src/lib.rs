//! # Bitbucket Server Rust Client
//!
//! `bitbucket-server-rs` is a Rust client library for interacting with the Bitbucket Data Center REST API.
//! This library provides a type-safe, ergonomic interface for Bitbucket Server (on-premise Bitbucket Data Center)
//! operations, making it ideal for automation tools, CI/CD integrations, and Rust applications that need to
//! interact with Bitbucket.
//!
//! ## Features
//!
//! - **Type-safe API**: Strongly typed interfaces for Bitbucket Server REST API endpoints
//! - **Async/Await Support**: Built on tokio and reqwest for modern asynchronous workflows
//! - **Error Handling**: Comprehensive error types for better error management
//! - **Builder Pattern**: Fluent API design for constructing requests
//! - **JSON Serialization/Deserialization**: Automatic handling of JSON payloads
//! - **Authentication**: Bearer token authentication support
//!
//! ## Currently Supported APIs
//!
//! - **Build Status**: Get and post build statuses for commits
//! - **Pull Request Changes**: Retrieve changes in pull requests
//!
//! ## Usage
//!
//! ### Creating a Client
//!
//! ```no_run
//! use bitbucket_server_rs::client::new;
//!
//! // Create a new client with base URL and API token
//! let client = new(
//!     "https://bitbucket-server/rest",
//!     "YOUR_API_TOKEN"
//! );
//! ```
//!
//! ### Getting Build Status
//!
//! ```no_run
//! use bitbucket_server_rs::client::{new, ApiRequest};
//! use bitbucket_server_rs::api::build_status_get::BuildStatus;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = new(
//!         "https://bitbucket-server/rest",
//!         "YOUR_API_TOKEN"
//!     );
//!
//!     // Get build status for a specific commit
//!     let response = client
//!         .api()
//!         .build_status_get(
//!             "PROJECT_KEY",
//!             "COMMIT_ID",
//!             "REPOSITORY_SLUG"
//!         )
//!         .key("build-123") // Optional: filter by build key
//!         .build()?
//!         .send()
//!         .await?;
//!
//!     // Handle the response
//!     match response {
//!         Some(build_status) => {
//!             println!("Build state: {:?}", build_status.state);
//!             println!("Build URL: {}", build_status.url);
//!             // Access other fields as needed
//!         },
//!         None => println!("No build status found")
//!     }
//!
//!     Ok(())
//! }
//! ```
//!
//! ### Posting Build Status
//!
//! ```no_run
//! use bitbucket_server_rs::client::new;
//! use bitbucket_server_rs::api::build_status::{BuildStatusState, TestResults};
//! use bitbucket_server_rs::api::build_status_post::BuildStatusPostPayload;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = new(
//!         "https://bitbucket-server/rest",
//!         "YOUR_API_TOKEN"
//!     );
//!
//!     // Create build status payload
//!     let build_status = BuildStatusPostPayload {
//!         key: "build-123".to_string(),
//!         state: BuildStatusState::Successful,
//!         url: "https://ci.example.com/build/123".to_string(),
//!         description: Some("Build passed successfully".to_string()),
//!         name: Some("CI Build".to_string()),
//!         ..Default::default()
//!     };
//!
//!     // Post a build status for a commit
//!     let response = client
//!         .api()
//!         .build_status_post(
//!             "PROJECT_KEY",
//!             "REPOSITORY_SLUG",
//!             "COMMIT_ID",
//!             &build_status
//!         )
//!         .send()
//!         .await?;
//!
//!     println!("Build status posted successfully");
//!
//!     Ok(())
//! }
//! ```
//!
//! ### Getting Pull Request Changes
//!
//! ```no_run
//! use bitbucket_server_rs::client::{new, ApiRequest};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = new(
//!         "https://bitbucket-server/rest",
//!         "YOUR_API_TOKEN"
//!     );
//!
//!     // Get changes for a pull request
//!     let response = client
//!         .api()
//!         .pull_request_changes_get(
//!             "PROJECT_KEY",
//!             "REPOSITORY_SLUG",
//!             "123" // Pull request ID
//!         )
//!         .start(0)
//!         .limit(100)
//!         .build()?
//!         .send()
//!         .await?;
//!
//!     // Handle the response
//!     if let Some(changes) = response {
//!         println!("Changes found!");
//!         // Process changes
//!     }
//!
//!     Ok(())
//! }
//! ```
//!
//! ## Error Handling
//!
//! The library provides a comprehensive error type `ApiError` that covers various failure scenarios:
//!
//! ```no_run
//! use bitbucket_server_rs::client::{new, ApiError, ApiRequest};
//!
//! #[tokio::main]
//! async fn main() {
//!     let client = new("https://bitbucket-server/rest", "API_TOKEN");
//!
//!     match client.api().build_status_get("PROJECT", "COMMIT", "REPO").build().unwrap().send().await {
//!         Ok(response) => {
//!             // Handle successful response
//!         },
//!         Err(ApiError::Unauthorized) => {
//!             eprintln!("Authentication failed. Check your API token.");
//!         },
//!         Err(ApiError::HttpClientError(status, message)) => {
//!             eprintln!("Client error ({}): {}", status, message);
//!         },
//!         Err(e) => {
//!             eprintln!("Request failed: {:?}", e);
//!         }
//!     }
//! }
//! ```

/// Bitbucket's `api` API module containing all API endpoint implementations
pub mod api;

/// REST API Client module providing the core client functionality
pub mod client;
