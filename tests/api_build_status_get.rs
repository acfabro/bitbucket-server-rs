use bitbucket_server_rs::client;
use httpmock::Method::POST;
use httpmock::MockServer;

#[tokio::test]
async fn it_can_get_build_status() {
    let server = MockServer::start();
    let mock = server.mock(|when, then| {
        when.method(POST)
            .path("/api/v1/repos/PROJECT_KEY/REPOSITORY_SLUG/commits/COMMIT_ID/build-status")
            .header("Content-Type", "application/json");
        then.body(r#"{"key":"KEY"}"#).status(200);
    });

    let client = client::new(
        server.url("").to_string(),
        reqwest::Client::new(),
        "API_TOKEN".to_string(),
    );

    let response = client
        .api()
        .get_build_status(
            "PROJECT_KEY".to_string(),
            "COMMIT_ID".to_string(),
            "REPOSITORY_SLUG".to_string(),
        )
        .send()
        .await;

    assert!(response.is_ok());
    mock.assert();
}
