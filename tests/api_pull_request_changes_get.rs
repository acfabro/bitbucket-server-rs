mod common;

use bitbucket_server_rs::client::ApiRequest;
use httpmock::Method::GET;

#[tokio::test]
async fn it_can_get_pull_request_changes() {
    // common::setup();
    let (server, client) = common::mock_client();

    let mock = server.mock(|when, then| {
        when.method(GET).path(
            "/rest/api/latest/projects/PROJECT_KEY/repos/REPOSITORY_SLUG/pull-requests/PULL_REQUEST_ID/changes"
        );
        then.status(200).body(r#"{
            "fromHash":"from_hash",
            "toHash":"to_hash",
            "values":[
                {"contentId":"12345","type":"ADD","path":{"toString":"path/to/file"}},
                {"contentId":"67890","type":"COPY","path":{"toString":"another/target"}}
            ]
        }"#);
    });

    let response = client
        .api()
        .get_pull_request_changes(
            "PROJECT_KEY".to_string(),
            "REPOSITORY_SLUG".to_string(),
            "PULL_REQUEST_ID".to_string(),
        )
        .send()
        .await;

    assert!(response.is_ok());
    mock.assert();
}

#[tokio::test]
async fn it_can_get_pull_request_changes_with_params() {
    // common::setup();
    let (server, client) = common::mock_client();

    let mock = server.mock(|when, then| {
        when.method(GET)
            .path("/rest/api/latest/projects/PROJECT_KEY/repos/REPOSITORY_SLUG/pull-requests/PULL_REQUEST_ID/changes")
            .query_param("changeScope", "SCOPE")
            .query_param("sinceId", "SINCE_ID")
            .query_param("untilId", "UNTIL_ID")
            .query_param("start", "1")
            .query_param("limit", "10")
            .query_param("withComments", "true");
        then.status(200)
            .body(r#"{
                "fromHash":"from_hash",
                "toHash":"to_hash",
                "values":[
                    {"contentId":"12345","type":"ADD","path":{"toString":"path/to/file"}},
                    {"contentId":"67890","type":"COPY","path":{"toString":"another/target"}}
                ]
            }"#);
    });

    let response = client
        .api()
        .get_pull_request_changes(
            "PROJECT_KEY".to_string(),
            "REPOSITORY_SLUG".to_string(),
            "PULL_REQUEST_ID".to_string(),
        )
        .change_scope("SCOPE")
        .since_id("SINCE_ID")
        .until_id("UNTIL_ID")
        .start(1)
        .limit(10)
        .with_comments(true)
        .send()
        .await;

    assert!(response.is_ok());
    mock.assert();
}
