use bitbucket_server_rs::api::build_status::*;
use bitbucket_server_rs::client;
use chrono::{DateTime, Utc};
use httpmock::Method::POST;
use httpmock::MockServer;
use serde_json::json;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[tokio::test]
async fn it_can_post_build_status() {
    // test that bitbucket_server_rs::client can post a build status

    let server = MockServer::start();
    let mock = server.mock(|when, then| {
        when.method(POST)
            .path("/api/v1/repos/PROJECT_KEY/REPOSITORY_SLUG/commits/COMMIT_ID/build-status")
            .header("Content-Type", "application/json")
            .json_body(json!({
                "key": "KEY",
                "state": "SUCCESSFUL",
                "url": "URL",
                "buildNumber": "1",
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
        then.status(200);
    });

    let client = client::new_client(
        server.url("").to_string(),
        reqwest::Client::new(),
        "API_TOKEN".to_string(),
    );

    let params = PostBuildStatusParams::new(
        "PROJECT_KEY".to_string(),
        "COMMIT_ID".to_string(),
        "REPOSITORY_SLUG".to_string(),
        BuildStatus::new(
            "KEY".to_string(),
            BuildStatusState::Successful,
            "URL".to_string(),
        )
        .with_build_number("1".to_string())
        .with_date_added(
            DateTime::parse_from_rfc3339("2025-01-30T01:02:03Z")
                .unwrap()
                .with_timezone(&Utc),
        )
        .with_duration_secs(12)
        .with_description("DESCRIPTION".to_string())
        .with_name("NAME".to_string())
        .with_parent("PARENT".to_string())
        .with_reference("REF".to_string())
        .with_test_results(3, 2, 1),
    );

    let result = client.api().post_build_status(params).await;
    assert!(result.is_ok());
    mock.assert();
}
