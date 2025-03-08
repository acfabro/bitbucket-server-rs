# bitbucket-server-rs

[![Rust](https://img.shields.io/badge/rust-stable-brightgreen.svg)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Crate](https://img.shields.io/crates/v/bitbucket-server-rs.svg)](https://crates.io/crates/bitbucket-server-rs)
[![API](https://docs.rs/bitbucket-server-rs/badge.svg)](https://docs.rs/bitbucket-server-rs)

A Rust client library for interacting with the Bitbucket Data Center REST API. This library provides a type-safe, ergonomic interface for Bitbucket Server (on-premise Bitbucket Data Center) operations, making it ideal for automation tools, CI/CD integrations, and Rust applications that need to interact with Bitbucket.

## Features

- **Type-safe API**: Strongly typed interfaces for Bitbucket Server REST API endpoints
- **Async/Await Support**: Built on tokio and reqwest for modern asynchronous workflows
- **Error Handling**: Comprehensive error types for better error management
- **Builder Pattern**: Fluent API design for constructing requests
- **JSON Serialization/Deserialization**: Automatic handling of JSON payloads
- **Authentication**: Bearer token authentication support

## Currently Supported APIs

- **Build Status**: Get and post build statuses for commits
- **Pull Request Changes**: Retrieve changes in pull requests

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
bitbucket-server-rs = "0.3.1"
tokio = { version = "1.0", features = ["full"] } # For async runtime
```

## Usage

### Creating a Client

```rust
use bitbucket_server_rs::client::new;

// Create a new client with base URL and API token
let client = new(
    "https://bitbucket-server/rest",
    "YOUR_API_TOKEN"
);
```

### Getting Build Status

```rust
use bitbucket_server_rs::client::{new, ApiError, ApiRequest, ApiResponse};
use bitbucket_server_rs::api::build_status_get::BuildStatus;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = new(
        "https://bitbucket-server/rest",
        "YOUR_API_TOKEN"
    );

    // Get build status for a specific commit
    let response = client
        .api()
        .build_status_get(
            "PROJECT_KEY",
            "COMMIT_ID",
            "REPOSITORY_SLUG"
        )
        .key("build-123") // Optional: filter by build key
        .build()?
        .send()
        .await?;

    // Handle the response
    match response {
        Some(build_status) => {
            println!("Build state: {:?}", build_status.state);
            println!("Build URL: {}", build_status.url);
            // Access other fields as needed
        },
        None => println!("No build status found")
    }

    Ok(())
}
```

### Posting Build Status

```rust
use bitbucket_server_rs::client::new;
use bitbucket_server_rs::api::build_status::BuildStatusState;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = new(
        "https://bitbucket-server/rest",
        "YOUR_API_TOKEN"
    );

    // Post a build status for a commit
    let response = client
        .api()
        .build_status_post(
            "PROJECT_KEY",
            "COMMIT_ID",
            "REPOSITORY_SLUG"
        )
        .key("build-123")
        .state(BuildStatusState::Successful)
        .url("https://ci.example.com/build/123")
        .description("Build passed successfully")
        .name("CI Build")
        .build()?
        .send()
        .await?;

    println!("Build status posted successfully");

    Ok(())
}
```

### Getting Pull Request Changes

```rust
use bitbucket_server_rs::client::new;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = new(
        "https://bitbucket-server/rest",
        "YOUR_API_TOKEN"
    );

    // Get changes for a pull request
    let response = client
        .api()
        .pull_request_changes_get(
            "PROJECT_KEY",
            "REPOSITORY_SLUG",
            123 // Pull request ID
        )
        .start(0)
        .limit(100)
        .build()?
        .send()
        .await?;

    // Handle the response
    if let Some(changes) = response {
        println!("Found {} changes", changes.values.len());
        for change in changes.values {
            println!("Change: {:?}", change.path);
        }
    }

    Ok(())
}
```

## Error Handling

The library provides a comprehensive error type `ApiError` that covers various failure scenarios:

```rust
pub enum ApiError {
    // Error building the request
    RequestError,
    
    // Error getting the response
    ResponseError,
    
    // Authentication error
    Unauthorized,
    
    // HTTP 4xx errors
    HttpClientError(u16, String),
    
    // HTTP 5xx errors
    HttpServerError(u16, String),
    
    // Unexpected response
    UnexpectedResponse(u16, String),
    
    // Error deserializing the response body
    DeserializationError(String),
}
```

Example of error handling:

```rust
match client.api().build_status_get("PROJECT", "COMMIT", "REPO").build()?.send().await {
    Ok(response) => {
        // Handle successful response
    },
    Err(ApiError::Unauthorized) => {
        eprintln!("Authentication failed. Check your API token.");
    },
    Err(ApiError::HttpClientError(status, message)) => {
        eprintln!("Client error ({}): {}", status, message);
    },
    Err(e) => {
        eprintln!("Request failed: {:?}", e);
    }
}
```

## Custom HTTP Client Configuration

You can customize the HTTP client configuration:

```rust
use bitbucket_server_rs::client::{Client, new};
use reqwest::ClientBuilder;

// Create a custom HTTP client
let http_client = ClientBuilder::new()
    .timeout(std::time::Duration::from_secs(30))
    .build()
    .expect("Failed to build HTTP client");

// Create a new Bitbucket client
let mut client = new("https://bitbucket-server/rest", "API_TOKEN");

// Set the custom HTTP client
client.with_http_client(http_client);
```

## CI/CD

This project uses GitHub Actions for continuous integration and deployment:

- **PR Check**: Runs tests and ensures version is bumped on pull requests
- **Publish**: Automatically publishes to crates.io when changes are merged to main

### Setting up for Publishing

To enable publishing to crates.io with manual approval, you need to:

1. Generate a new token on [crates.io](https://crates.io/me/tokens)
2. Go to your GitHub repository settings → Secrets and variables → Actions
3. Add a new repository secret named `CRATES_IO_TOKEN` with your crates.io API token as the value
4. Go to Settings → Environments → New environment
5. Create an environment named `crates-io-publish`
6. Add required reviewers who must approve the publishing step

With this setup, the workflow will:
1. Automatically prepare the release when changes are merged to main
2. Run build and tests to verify everything works
3. Wait for manual approval from the required reviewers
4. After approval, publish to crates.io, which triggers docs.rs to build and publish the documentation

You can also manually trigger the publishing workflow from the Actions tab in GitHub.

## Contributing

Contributions are welcome! Here's how you can contribute:

1. Fork the repository
2. Create a feature branch: `git checkout -b feature-name`
3. Commit your changes: `git commit -am 'Add some feature'`
4. Push to the branch: `git push origin feature-name`
5. Submit a pull request

Please make sure to update tests as appropriate and follow the Rust code style guidelines.

### Version Bumping

When making changes, remember to bump the version in `Cargo.toml` according to [Semantic Versioning](https://semver.org/) principles:

- **MAJOR** version for incompatible API changes
- **MINOR** version for adding functionality in a backwards compatible manner
- **PATCH** version for backwards compatible bug fixes

The CI workflow will verify that the version has been bumped in pull requests.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- [reqwest](https://github.com/seanmonstar/reqwest) - HTTP client for Rust
- [serde](https://github.com/serde-rs/serde) - Serialization framework for Rust
- [tokio](https://github.com/tokio-rs/tokio) - Asynchronous runtime for Rust
