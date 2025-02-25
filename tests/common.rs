use bitbucket_server_rs::client;
use httpmock::MockServer;

#[allow(dead_code)]
pub fn mock_client() -> (MockServer, client::Client) {
    let server = MockServer::start();
    let client = client::new(&server.url("/rest"), "API_TOKEN");

    (server, client)
}
