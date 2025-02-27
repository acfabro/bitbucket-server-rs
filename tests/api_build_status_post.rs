mod common;

use bitbucket_server_rs::client;
use bitbucket_server_rs::client::ApiRequest;
use chrono::{DateTime, Utc};
use httpmock::Method::POST;
use httpmock::MockServer;
use serde_json::json;

#[tokio::test]
async fn it_can_post_build_status() {
    common::setup();

    let (server, client) = mock_client();

    let mock = server.mock(|when, then| {
        when.method(POST)
            .path("/rest/api/latest/projects/PROJECT_KEY/repos/REPOSITORY_SLUG/commits/COMMIT_ID/builds")
            .json_body(json!({
                "key": "KEY",
                "state": "SUCCESSFUL",
                "url": "https://my-build-status.com/path",
                "buildNumber": "9",
                "dateAdded": 1738198923,
                "duration": 12,
                "description": "DESCRIPTION",
                "name": "NAME",
                "parent": "PARENT",
                "ref": "REF",
                "testResults": {
                    "failed": 2,
                    "successful": 3,
                    "skipped": 1
                }
            }));
        then.status(204);
    });

    let result = client
        .api()
        .build_status_post(
            "PROJECT_KEY".to_string(),
            "COMMIT_ID".to_string(),
            "REPOSITORY_SLUG".to_string(),
            "KEY".to_string(),
            "https://my-build-status.com/path".to_string(),
        )
        .state_successful()
        .build_number("9".to_string())
        .date_added(
            DateTime::parse_from_rfc3339("2025-01-30T01:02:03Z")
                .unwrap()
                .with_timezone(&Utc),
        )
        .duration_secs(12)
        .description("DESCRIPTION".to_string())
        .name("NAME".to_string())
        .parent("PARENT".to_string())
        .reference("REF".to_string())
        .test_results(3, 2, 1)
        .send()
        .await;

    assert!(result.is_ok());
    mock.assert();
}

fn mock_client() -> (MockServer, client::Client) {
    let server = MockServer::start();
    let client = client::new(&server.url("/rest"), "API_TOKEN");

    (server, client)
}
