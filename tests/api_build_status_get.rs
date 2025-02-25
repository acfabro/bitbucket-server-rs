mod common;

use bitbucket_server_rs::api::build_status::BuildStatusState;
use bitbucket_server_rs::client::ApiRequest;
use httpmock::Method::GET;

#[tokio::test]
async fn it_can_get_build_status() {
    common::setup();
    let (server, client) = common::mock_client();

    let mock = server.mock(|when, then| {
        when.method(GET).path(
            "/rest/api/latest/projects/PROJECT_KEY/repos/REPOSITORY_SLUG/commits/COMMIT_ID/builds",
        );
        then.body(TEST_RESPONSE).status(200);
    });

    let response = client
        .api()
        .build_status_get(
            "PROJECT_KEY".to_string(),
            "COMMIT_ID".to_string(),
            "REPOSITORY_SLUG".to_string(),
        )
        .send()
        .await;

    assert!(response.is_ok());
    mock.assert();
}

#[tokio::test]
async fn it_can_get_build_status_with_key() {
    common::setup();
    let (server, client) = common::mock_client();

    let mock = server.mock(|when, then| {
        when.method(GET)
            .path("/rest/api/latest/projects/PROJECT_KEY/repos/REPOSITORY_SLUG/commits/COMMIT_ID/builds")
            .query_param("key", "ABC123");
        then.body(TEST_RESPONSE).status(200);
    });

    let response = client
        .api()
        .build_status_get(
            "PROJECT_KEY".to_string(),
            "COMMIT_ID".to_string(),
            "REPOSITORY_SLUG".to_string(),
        )
        .key("ABC123")
        .send()
        .await;

    assert!(response.is_ok());
    mock.assert();
}

#[tokio::test]
async fn it_can_get_build_status_object() {
    common::setup();
    let (server, client) = common::mock_client();

    let mock = server.mock(|when, then| {
        when.method(GET).path(
            "/rest/api/latest/projects/PROJECT_KEY/repos/REPOSITORY_SLUG/commits/COMMIT_ID/builds",
        );
        then.body(TEST_RESPONSE).status(200);
    });

    let response = client
        .api()
        .build_status_get(
            "PROJECT_KEY".to_string(),
            "COMMIT_ID".to_string(),
            "REPOSITORY_SLUG".to_string(),
        )
        .send()
        .await;

    // Check the response's fields
    let build_status = response.unwrap().unwrap();
    assert_eq!(build_status.name, Some("Database Matrix Tests".to_string()));
    assert_eq!(build_status.key, "TEST-REP3".to_string());
    assert_eq!(build_status.parent, Some("TEST-REP".to_string()));
    assert_eq!(build_status.state, BuildStatusState::Cancelled);
    assert_eq!(
        build_status.reference,
        Some("refs/heads/master".to_string())
    );
    mock.assert();
}

static TEST_RESPONSE: &str = r#"{
  "name": "Database Matrix Tests",
  "key": "TEST-REP3",
  "parent": "TEST-REP",
  "state": "CANCELLED",
  "ref": "refs/heads/master",
  "testResults": {
    "failed": 1,
    "successful": 134,
    "skipped": 5
  },
  "createdDate": 1738198923,
  "updatedDate": 1738198924,
  "url": "https://my-bitbucket-server.com/browse/TEST-REP3",
  "duration": 2154,
  "buildNumber": "3",
  "description": "A description of the build goes here"
}"#;
