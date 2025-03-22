mod common;

use bitbucket_server_rs::api::pull_request_post::{
    PullRequestPostPayload, ProjectInfo, RefInfo, RepositoryInfo, Reviewer, User,
};
use bitbucket_server_rs::client::ApiRequest;
use httpmock::Method::POST;
use serde_json::json;

#[tokio::test]
async fn it_can_create_pull_request() {
    common::setup();

    let (server, client) = common::mock_client();

    let mock = server.mock(|when, then| {
        when.method(POST)
            .path("/rest/api/latest/projects/PROJECT_KEY/repos/REPOSITORY_SLUG/pull-requests")
            .json_body(json!({
                "title": "Add new feature",
                "description": "Implements the new feature",
                "fromRef": {
                    "id": "refs/heads/feature-branch",
                    "repository": {
                        "slug": "my-repo",
                        "project": {
                            "key": "PROJECT_KEY"
                        }
                    }
                },
                "toRef": {
                    "id": "refs/heads/main",
                    "repository": {
                        "slug": "my-repo",
                        "project": {
                            "key": "PROJECT_KEY"
                        }
                    }
                },
                "reviewers": [
                    {
                        "user": {
                            "name": "reviewer1"
                        }
                    }
                ]
            }));
        then.status(201)
            .json_body(json!({
                "title": "Add new feature",
                "description": "Implements the new feature",
                "fromRef": {
                    "id": "refs/heads/feature-branch",
                    "repository": {
                        "slug": "my-repo",
                        "project": {
                            "key": "PROJECT_KEY"
                        }
                    }
                },
                "toRef": {
                    "id": "refs/heads/main",
                    "repository": {
                        "slug": "my-repo",
                        "project": {
                            "key": "PROJECT_KEY"
                        }
                    }
                },
                "reviewers": [
                    {
                        "user": {
                            "name": "reviewer1"
                        }
                    }
                ]
            }));
    });

    let repository_info = RepositoryInfo {
        slug: "my-repo".to_string(),
        project: ProjectInfo {
            key: "PROJECT_KEY".to_string(),
        },
    };

    let result = client
        .api()
        .pull_request_post(
            "PROJECT_KEY",
            "REPOSITORY_SLUG",
            &PullRequestPostPayload {
                title: "Add new feature".to_string(),
                description: Some("Implements the new feature".to_string()),
                from_ref: RefInfo {
                    id: "refs/heads/feature-branch".to_string(),
                    repository: repository_info.clone(),
                },
                to_ref: RefInfo {
                    id: "refs/heads/main".to_string(),
                    repository: repository_info,
                },
                reviewers: Some(vec![Reviewer {
                    user: User {
                        name: "reviewer1".to_string(),
                    },
                }]),
            },
        )
        .send()
        .await;

    assert!(result.is_ok());
    mock.assert();
}

#[tokio::test]
async fn it_can_create_pull_request_without_optional_fields() {
    common::setup();

    let (server, client) = common::mock_client();

    let mock = server.mock(|when, then| {
        when.method(POST)
            .path("/rest/api/latest/projects/PROJECT_KEY/repos/REPOSITORY_SLUG/pull-requests")
            .json_body(json!({
                "title": "Add new feature",
                "fromRef": {
                    "id": "refs/heads/feature-branch",
                    "repository": {
                        "slug": "my-repo",
                        "project": {
                            "key": "PROJECT_KEY"
                        }
                    }
                },
                "toRef": {
                    "id": "refs/heads/main",
                    "repository": {
                        "slug": "my-repo",
                        "project": {
                            "key": "PROJECT_KEY"
                        }
                    }
                }
            }));
        then.status(201)
            .json_body(json!({
                "title": "Add new feature",
                "fromRef": {
                    "id": "refs/heads/feature-branch",
                    "repository": {
                        "slug": "my-repo",
                        "project": {
                            "key": "PROJECT_KEY"
                        }
                    }
                },
                "toRef": {
                    "id": "refs/heads/main",
                    "repository": {
                        "slug": "my-repo",
                        "project": {
                            "key": "PROJECT_KEY"
                        }
                    }
                }
            }));
    });

    let repository_info = RepositoryInfo {
        slug: "my-repo".to_string(),
        project: ProjectInfo {
            key: "PROJECT_KEY".to_string(),
        },
    };

    let result = client
        .api()
        .pull_request_post(
            "PROJECT_KEY",
            "REPOSITORY_SLUG",
            &PullRequestPostPayload {
                title: "Add new feature".to_string(),
                description: None,
                from_ref: RefInfo {
                    id: "refs/heads/feature-branch".to_string(),
                    repository: repository_info.clone(),
                },
                to_ref: RefInfo {
                    id: "refs/heads/main".to_string(),
                    repository: repository_info,
                },
                reviewers: None,
            },
        )
        .send()
        .await;

    assert!(result.is_ok());
    mock.assert();
}
