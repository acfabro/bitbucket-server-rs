# bitbucket-server-rs

[![Rust](https://img.shields.io/badge/rust-stable-brightgreen.svg)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Crate](https://img.shields.io/crates/v/bitbucket-server-rs.svg)](https://crates.io/crates/bitbucket-server-rs)
[![API](https://docs.rs/bitbucket-server-rs/badge.svg)](https://docs.rs/bitbucket-server-rs)

A Rust client library for interacting with the Bitbucket Data Center REST API. This library provides a type-safe,
ergonomic interface for Bitbucket Server (on-premise Bitbucket Data Center) operations, making it ideal for automation
tools, CI/CD integrations, and Rust applications that need to interact with Bitbucket.

# NOTE

**At the moment this repo is used for learning purposes, so use it at your own risk. However, I
do plan to use it in a real project in the near future, so I will be happy to accept contributions
if you find it useful.**

## Features

- **Type-safe API**: Strongly typed interfaces for Bitbucket Server REST API endpoints
- **Async/Await Support**: Built on tokio and reqwest for modern asynchronous workflows
- **Error Handling**: Comprehensive error types for better error management
- **Builder Pattern**: Fluent API design for constructing requests
- **JSON Serialization/Deserialization**: Automatic handling of JSON payloads

## Currently Supported APIs

- **Build Status**: Get and post build statuses for commits
- **Pull Request Changes**: Retrieve changes in pull requests
- **Pull Request Creation**: Create new pull requests

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
bitbucket-server-rs = "0.4.1"
tokio = { version = "1.0", features = ["full"] } # For async runtime
```

## Usage

### Creating a Client

```no_run
use bitbucket_server_rs::client::new;

// Create a new client with base URL and API token
let client = new(
    "https://bitbucket-server/rest",  // Required: Base URL of your Bitbucket server
    "YOUR_API_TOKEN"                  // Required: API token for authentication
);
```

### Getting Build Status

```rust
use bitbucket_server_rs::Error;
use bitbucket_server_rs::client::{new, ApiRequest, ApiResponse};
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
            "PROJECT_KEY",     // Required: Project key
            "COMMIT_ID",       // Required: Commit hash
            "REPOSITORY_SLUG"  // Required: Repository slug
        )
        .key("build-123")     // Optional: Filter by build key
        .build()?
        .send()
        .await?;

    // Handle the response
    match response {
        Some(build_status) => {
            println!("Build state: {:?}", build_status.state);
            println!("Build URL: {}", build_status.url);
            // Access other fields as needed
        }
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
            "PROJECT_KEY",     // Required: Project key
            "COMMIT_ID",       // Required: Commit hash
            "REPOSITORY_SLUG"  // Required: Repository slug
        )
        .key("build-123")           // Required: Unique identifier for this build status
        .state(BuildStatusState::Successful)  // Required: Build state (Successful/Failed/InProgress)
        .url("https://ci.example.com/build/123")  // Required: URL to build details
        .description("Build passed successfully")  // Optional: Description of the build status
        .name("CI Build")                         // Optional: Display name for this build
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
            "PROJECT_KEY",     // Required: Project key
            "REPOSITORY_SLUG", // Required: Repository slug
            123               // Required: Pull request ID
        )
        .start(0)            // Optional: Starting position of the page (default: 0)
        .limit(100)          // Optional: Maximum number of changes to return (default: 100)
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

### Creating a Pull Request

```rust
use bitbucket_server_rs::client::new;
use bitbucket_server_rs::api::pull_request_post::{
    PullRequestPostPayload, RefInfo, RepositoryInfo, ProjectInfo, Reviewer, User
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = new(
        "https://bitbucket-server/rest",
        "YOUR_API_TOKEN"
    );

    // Create repository info (reused for both from and to refs)
    let repository_info = RepositoryInfo {
        slug: "my-repo".to_string(),
        project: ProjectInfo {
            key: "PROJECT_KEY".to_string(),
        },
    };

    // Create pull request payload
    let pull_request = PullRequestPostPayload {
        title: "Add new feature".to_string(),                // Required: PR title
        description: Some("Implements the new feature".to_string()), // Optional: PR description
        from_ref: RefInfo {
            id: "refs/heads/feature-branch".to_string(),     // Required: Source branch
            repository: repository_info.clone(),
        },
        to_ref: RefInfo {
            id: "refs/heads/main".to_string(),               // Required: Target branch
            repository: repository_info,
        },
        reviewers: Some(vec![Reviewer {                      // Optional: PR reviewers
            user: User {
                name: "reviewer1".to_string(),
            },
        }]),
    };

    // Create the pull request
    let response = client
        .api()
        .pull_request_post(
            "PROJECT_KEY",     // Required: Project key
            "my-repo",         // Required: Repository slug
            &pull_request
        )
        .send()
        .await?;

    println!("Pull request created successfully");

    Ok(())
}
```

## Custom HTTP Client Configuration

You can customize the HTTP client configuration:

```no_run
use bitbucket_server_rs::client::{Client, new};
use reqwest::ClientBuilder;

// Create a custom HTTP client
let http_client = ClientBuilder::new()
    .timeout(std::time::Duration::from_secs(30))  // Optional: Set custom timeout
    .build()
    .expect("Failed to build HTTP client");

// Create a new Bitbucket client
let mut client = new("https://bitbucket-server/rest", "API_TOKEN");

// Set the custom HTTP client (optional)
client.with_http_client(http_client);
```

## CI/CD

This project uses GitHub Actions for continuous integration and deployment:

- **PR Check**: Runs tests and ensures version is bumped on pull requests
- **Publish**: Automatically publishes to crates.io when changes are merged to main

## Contributing

1. Fork the repository
2. Create a feature branch: `git checkout -b feature-name`
3. Commit your changes: `git commit -am 'Add some feature'`
4. Push to the branch: `git push origin feature-name`
5. Submit a pull request

Please make sure to update tests as appropriate and follow the Rust code style guidelines.

### Version Bumping

When making changes, remember to bump the version in `Cargo.toml` according
to [Semantic Versioning](https://semver.org/) principles:

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
