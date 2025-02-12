use bitbucket_server_rs::client;
use httpmock::MockServer;

#[tokio::test]
async fn it_can_get_pull_request_changes() {
    let server = MockServer::start();
    let mock = server.mock(|when, then| {
        when.method(httpmock::Method::GET)
            .path("/api/latest/projects/PROJECT_KEY/repos/REPOSITORY_SLUG/pull-requests/PULL_REQUEST_ID/changes")
            .header("Content-Type", "application/json");
        then
            .status(200)
            .body(r#"{
                "fromHash":"from_hash",
                "toHash":"to_hash",
                "values":[
                    {"contentId":"12345","type":"ADD","path":{"toString":"path/to/file"}},
                    {"contentId":"67890","type":"COPY","path":{"toString":"another/target"}}
                ]
            }"#);
    });

    let client = client::new(
        server.url("").to_string(),
        reqwest::Client::new(),
        "API_TOKEN".to_string(),
    );

    let response = client
        .api()
        .get_pull_request_changes(
            "PROJECT_KEY".to_string(),
            "PULL_REQUEST_ID".to_string(),
            "REPOSITORY_SLUG".to_string(),
            "PATH".to_string(),
        )
        .send()
        .await;

    assert!(response.is_ok());
    mock.assert();
}
