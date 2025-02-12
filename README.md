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
        // using the `api` API
        .api()
        // get the list of changes of a pull request
        .get_pull_request_changes(
            "MYPROJECT".to_string(),
            "42".to_string(),
            "repository-one".to_string(),
            "src/".to_string()
        )
        .send()
        .await;
    
    println!("Got response: {:?}", response);
}
```

# Notes 
* This project is used for learning purposes as of now.
