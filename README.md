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

## Currently Supported APIs

- **Build Status**: Get and post build statuses for commits
- **Pull Request Changes**: Retrieve changes in pull requests
- **Pull Request Creation**: Create new pull requests

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
bitbucket-server-rs = "0.5.1"
tokio = { version = "1.0", features = ["full"] } # For async runtime
```

## Usage and Examples

### Creating a Client

```no_run
use bitbucket_server_rs::new;

// Create a new client with base URL and API token
let client = new(
    "https://bitbucket-server/rest",  // Required: Base URL of your Bitbucket server
    "YOUR_API_TOKEN"                  // Required: API token for authentication
);
```

### Using the Prelude

For convenience, you can import everything you need from the prelude module:

```rust
use bitbucket_server_rs::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = new(
        "https://bitbucket-server/rest",
        "YOUR_API_TOKEN"
    );
    
    // Now you can use all the imported types without additional imports
    // ...
    
    Ok(())
}
```

### Example: Posting Build Status

```rust
use bitbucket_server_rs::{new, ApiRequest};
use bitbucket_server_rs::api::build_status::BuildStatusState;
use bitbucket_server_rs::api::build_status_post::BuildStatusPostPayload;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = new(
        "https://bitbucket-server/rest",
        "YOUR_API_TOKEN"
    );

    // Create build status payload
    let build_status = BuildStatusPostPayload {
        key: "build-123".to_string(),
        state: BuildStatusState::Successful,
        url: "https://ci.example.com/build/123".to_string(),
        description: Some("Build passed successfully".to_string()),
        name: Some("CI Build".to_string()),
        ..Default::default()
    };

    // Post a build status for a commit
    let response = client
        .api()
        .build_status_post(
            "PROJECT_KEY",     // Required: Project key
            "REPOSITORY_SLUG", // Required: Repository slug
            "COMMIT_ID",       // Required: Commit hash
            &build_status
        )
        .send()
        .await?;

    println!("Build status posted successfully");

    Ok(())
}
```

### Example: Getting Pull Request Changes

```rust
use bitbucket_server_rs::{new, ApiRequest};
use bitbucket_server_rs::api::pull_request_changes_get::{PullRequestChanges, ChangeItem, Path};

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
        println!("Found {} changes", changes.values.unwrap_or_default().len());
        for change in changes.values.unwrap_or_default() {
            println!("Change type: {}, Path: {}", change.change_type, change.path.to_string);
        }
    }

    Ok(())
}
```

### Example: Creating a Pull Request

```rust
use bitbucket_server_rs::{new, ApiRequest};
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

### Error Handling

The library provides helper methods for error handling:

```rust
use bitbucket_server_rs::{Error, new, ApiRequest};

#[tokio::main]
async fn main() {
    let client = new("https://bitbucket-server/rest", "API_TOKEN");

    match client.api().build_status_get("PROJECT", "COMMIT", "REPO").build().unwrap().send().await {
        Ok(response) => {
            // Handle successful response
        },
        Err(e) if e.is_unauthorized() => {
            eprintln!("Authentication failed. Check your API token.");
        },
        Err(e) if e.is_request_error() => {
            eprintln!("Request error: {}", e);
        },
        Err(e) => {
            eprintln!("Other error: {}", e);
        }
    }
}
```

## Custom HTTP Client Configuration

You can customize the HTTP client configuration:

```no_run
use bitbucket_server_rs::{Client, new};
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

**At the moment this repo is used for learning purposes, so use it at your own risk. However, I
do plan to use it in a real project in the near future, so I will be happy to accept contributions
if you find it useful.**

### Version Bumping

**Note**: In versions `0.x.y`, every new minor version may introduce breaking changes in the API.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- [reqwest](https://github.com/seanmonstar/reqwest) - HTTP client for Rust
- [serde](https://github.com/serde-rs/serde) - Serialization framework for Rust
- [tokio](https://github.com/tokio-rs/tokio) - Asynchronous runtime for Rust
