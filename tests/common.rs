use bitbucket_server_rs::client;
use httpmock::MockServer;
use std::sync::Once;

static INIT: Once = Once::new();

pub fn setup() {
    INIT.call_once(|| {
        env_logger::init();
    });
}

#[allow(dead_code)]
pub fn mock_client() -> (MockServer, client::Client) {
    let server = MockServer::start();
    let client = client::new(&server.url("/rest"), "API_TOKEN");

    (server, client)
}
