mod common;

use bitbucket_server_rs::api::build_status::{BuildStatusState, TestResults};
use bitbucket_server_rs::api::build_status_post::BuildStatusPostPayload;
use bitbucket_server_rs::client::ApiRequest;
use chrono::{DateTime, Utc};
use httpmock::Method::POST;
use serde_json::json;

#[tokio::test]
async fn it_can_post_build_status() {
    common::setup();

    let (server, client) = common::mock_client();

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
            "PROJECT_KEY",
            "COMMIT_ID",
            "REPOSITORY_SLUG",
            &BuildStatusPostPayload {
                state: BuildStatusState::Successful,
                key: "KEY".to_string(),
                url: "https://my-build-status.com/path".to_string(),
                build_number: Some("9".to_string()),
                date_added: Some(
                    DateTime::parse_from_rfc3339("2025-01-30T01:02:03Z")
                        .unwrap()
                        .with_timezone(&Utc),
                ),
                duration: Some(12),
                description: Some("DESCRIPTION".to_string()),
                name: Some("NAME".to_string()),
                parent: Some("PARENT".to_string()),
                reference: Some("REF".to_string()),
                test_results: Some(TestResults {
                    successful: 3,
                    failed: 2,
                    skipped: 1,
                }),
            },
        )
        .send()
        .await;

    assert!(result.is_ok());
    mock.assert();
}
