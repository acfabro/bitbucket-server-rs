# bitbucket-server-rs

Bindings for Bitbucket Data Center REST API written in Rust.

## Usage

```rust
use bitbucket_server_rs::client;

async fn main() {
    let client = client::new(
        server.url("https://my-bitbucket-server.com/rest").to_string(),
        reqwest::Client::new(),
        "abc123def456".to_string(),
    );

    let response = client
        .api()
        .get_pull_request_changes(
            "PROJECT_KEY".to_string(),
            "PULL_REQUEST_ID".to_string(),
            "REPOSITORY_SLUG".to_string(),
        )
        .send()
        .await;
    
    println!("Got response: {:?}", response);
}
```

# Notes 
* This project is used for learning purposes as of now.
