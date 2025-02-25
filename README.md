# bitbucket-server-rs

Bindings for Bitbucket Data Center REST API written in Rust.

## Usage

```rust
use bitbucket_server_rs::client::{new, ApiError, ApiRequest, ApiResponse};
use bitbucket_server_rs::api::build_status_get::BuildStatus;

#[tokio::main]
async fn main() {
    let client = new(
        "https://bitbucket-server/rest",
        "API_TOKEN"
    );
    
    let response = client
        .api()
        .get_build_status(
            "PROJECT_KEY".to_string(),
            "COMMIT_ID".to_string(),
            "REPOSITORY_SLUG".to_string(),
        )
        .key("ABC123")
        .send()
        .await;
    
    // Handle the response
    println!("{:?}", response);
}
```

# Notes 
* This project is used for learning purposes as of now.
