use crate::api::Api;
use crate::client::Client;
use chrono::{serde::ts_seconds_option, DateTime, Utc};
use log::debug;
use reqwest::{Method, StatusCode};
use serde::{Deserialize, Serialize};
use serde_json::json;

/// The POST request payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
struct BuildStatusPostPayload {
    /// The string referring to this branch plan/job
    key: String,

    /// The build status state
    state: BuildStatusState,

    /// URL referring to the build result page in the CI tool.
    url: String,

    /// A unique identifier for this particular run of a plan
    #[serde(rename = "buildNumber", skip_serializing_if = "Option::is_none")]
    build_number: Option<String>,

    ///
    #[serde(
        rename = "dateAdded",
        with = "ts_seconds_option",
        skip_serializing_if = "Option::is_none",
        default
    )]
    date_added: Option<DateTime<Utc>>,

    /// Describes the build result
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,

    /// Duration of a completed build
    #[serde(skip_serializing_if = "Option::is_none")]
    duration: Option<u64>,

    /// A short string that describes the build plan
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,

    /// The identifier for the plan or job that ran the branch plan that produced this build status.
    #[serde(skip_serializing_if = "Option::is_none")]
    parent: Option<String>,

    /// The fully qualified git reference e.g. refs/heads/master.
    #[serde(rename = "ref", skip_serializing_if = "Option::is_none")]
    reference: Option<String>,

    /// A summary of the passed, failed and skipped tests.
    #[serde(rename = "testResults", skip_serializing_if = "Option::is_none")]
    test_results: Option<TestResults>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum BuildStatusState {
    #[serde(rename = "SUCCESSFUL")]
    Successful,
    #[serde(rename = "FAILED")]
    Failed,
    #[serde(rename = "IN_PROGRESS")]
    InProgress,
    #[serde(rename = "CANCELLED")]
    Cancelled,
    #[serde(rename = "UNKNOWN")]
    Unknown,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub(super) struct TestResults {
    pub(super) failed: u32,
    pub(super) successful: u32,
    pub(super) skipped: u32,
}

#[derive(Debug)]
pub struct BuildStatusPostBuilder {
    client: Client,
    project_key: String,
    commit_id: String,
    repository_slug: String,
    build_status: BuildStatusPostPayload,
}

impl BuildStatusPostBuilder {
    pub fn build_number(mut self, build_number: String) -> BuildStatusPostBuilder {
        self.build_status.build_number = Some(build_number);
        self
    }

    pub fn date_added(mut self, date_added: DateTime<Utc>) -> BuildStatusPostBuilder {
        self.build_status.date_added = Some(date_added);
        self
    }

    pub fn description(mut self, description: String) -> BuildStatusPostBuilder {
        self.build_status.description = Some(description);
        self
    }

    pub fn duration_secs(mut self, duration: u64) -> BuildStatusPostBuilder {
        self.build_status.duration = Some(duration);
        self
    }

    pub fn name(mut self, name: String) -> BuildStatusPostBuilder {
        self.build_status.name = Some(name);
        self
    }

    pub fn parent(mut self, parent: String) -> BuildStatusPostBuilder {
        self.build_status.parent = Some(parent);
        self
    }

    pub fn reference(mut self, reference: String) -> BuildStatusPostBuilder {
        self.build_status.reference = Some(reference);
        self
    }

    pub fn test_results(
        mut self,
        successful: u32,
        failed: u32,
        skipped: u32,
    ) -> BuildStatusPostBuilder {
        self.build_status.test_results = Some(TestResults {
            successful,
            failed,
            skipped,
        });
        self
    }

    pub fn state_successful(mut self) -> BuildStatusPostBuilder {
        self.build_status.state = BuildStatusState::Successful;
        self
    }
    pub fn state_failed(mut self) -> BuildStatusPostBuilder {
        self.build_status.state = BuildStatusState::Failed;
        self
    }
    pub fn state_in_progress(mut self) -> BuildStatusPostBuilder {
        self.build_status.state = BuildStatusState::InProgress;
        self
    }
    pub fn state_cancelled(mut self) -> BuildStatusPostBuilder {
        self.build_status.state = BuildStatusState::Cancelled;
        self
    }
    pub fn state_unknown(mut self) -> BuildStatusPostBuilder {
        self.build_status.state = BuildStatusState::Unknown;
        self
    }

    pub async fn send(&self) -> Result<(), String> {
        let api_client = self;

        let request_uri = format!(
            "{}/api/v1/repos/{}/{}/commits/{}/build-status",
            api_client.client.base_path, self.project_key, self.repository_slug, self.commit_id
        );

        let http_client = &api_client.client.client;
        let request_builder = http_client
            .request(Method::POST, request_uri.as_str())
            .header("Content-Type", "application/json");

        debug!("post_build_status: {}", json!(self.build_status));

        let request = request_builder.json(&self.build_status).build().unwrap();
        let response = http_client.execute(request).await.unwrap();

        match response.status() {
            //
            StatusCode::NO_CONTENT => Ok(()),
            //
            _ => Err(format!(
                "Unexpected Response [{}]: {}",
                response.status(),
                response.text().await.unwrap()
            )),
        }
    }
}

impl Api {
    /// Store a build status
    ///
    /// The authenticated user must have REPO_READ permission for the repository that this build
    /// status is for. The request can also be made with anonymous 2-legged OAuth.
    ///
    /// [Bitbucket Docs](https://developer.atlassian.com/server/bitbucket/rest/v811/api-group-builds-and-deployments/#api-api-latest-projects-projectkey-repos-repositoryslug-commits-commitid-builds-post)
    pub fn post_build_status(
        self,
        project_key: String,
        commit_id: String,
        repository_slug: String,
        build_status_key: String,
        build_status_url: String,
    ) -> BuildStatusPostBuilder {
        BuildStatusPostBuilder {
            client: self.client,
            project_key,
            commit_id,
            repository_slug,
            build_status: BuildStatusPostPayload {
                key: build_status_key,
                state: BuildStatusState::Unknown,
                url: build_status_url,
                build_number: None,
                date_added: None,
                description: None,
                duration: None,
                name: None,
                parent: None,
                reference: None,
                test_results: None,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::from_str;

    #[test]
    fn it_can_serialize() {
        let build_status = BuildStatusPostPayload {
            key: "KEY".to_string(),
            state: super::BuildStatusState::Successful,
            url: "URL".to_string(),
            build_number: Some("1".to_string()),
            date_added: Some(
                chrono::DateTime::parse_from_rfc3339("2025-01-30T01:02:03Z")
                    .unwrap()
                    .with_timezone(&chrono::Utc),
            ),
            description: Some("DESCRIPTION".to_string()),
            duration: Some(12),
            name: Some("NAME".to_string()),
            parent: Some("PARENT".to_string()),
            reference: Some("REF".to_string()),
            test_results: Some(super::TestResults {
                failed: 2,
                successful: 3,
                skipped: 1,
            }),
        };

        let json = serde_json::to_string(&build_status).unwrap();
        assert_eq!(
            json,
            r#"{"key":"KEY","state":"SUCCESSFUL","url":"URL","buildNumber":"1","dateAdded":1738198923,"description":"DESCRIPTION","duration":12,"name":"NAME","parent":"PARENT","ref":"REF","testResults":{"failed":2,"successful":3,"skipped":1}}"#
        );
    } // it_can_serialize

    #[test]
    fn it_can_deserialize_all_fields() {
        let json = r#"{
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
          "dateAdded": 1738198923,
          "url": "https://my-bitbucket-server.com/browse/TEST-REP3",
          "duration": 2154,
          "buildNumber": "3",
          "description": "A description of the build goes here"
        }"#;

        let build_status: BuildStatusPostPayload = from_str(json).unwrap();

        assert_eq!(
            build_status,
            BuildStatusPostPayload {
                key: "TEST-REP3".to_string(),
                state: super::BuildStatusState::Cancelled,
                url: "https://my-bitbucket-server.com/browse/TEST-REP3".to_string(),
                build_number: Some("3".to_string()),
                date_added: Some(
                    chrono::DateTime::parse_from_rfc3339("2025-01-30T01:02:03Z")
                        .unwrap()
                        .with_timezone(&chrono::Utc)
                ),
                description: Some("A description of the build goes here".to_string()),
                duration: Some(2154),
                name: Some("Database Matrix Tests".to_string()),
                parent: Some("TEST-REP".to_string()),
                reference: Some("refs/heads/master".to_string()),
                test_results: Some(super::TestResults {
                    failed: 1,
                    successful: 134,
                    skipped: 5
                })
            }
        );
    } // it_can_deserialize_all_fields

    #[test]
    fn it_can_deserialize_with_optional_fields() {
        let json = r#"{
          "key": "TEST-REP3",
          "state": "SUCCESSFUL",
          "url": "https://my-bitbucket-server.com/browse/TEST-REP3",
          "buildNumber": "10"
        }"#;

        let build_status: BuildStatusPostPayload = from_str(json).unwrap();
        assert_eq!(
            build_status,
            BuildStatusPostPayload {
                key: "TEST-REP3".to_string(),
                state: super::BuildStatusState::Successful,
                url: "https://my-bitbucket-server.com/browse/TEST-REP3".to_string(),
                build_number: Some("10".to_string()),
                date_added: None,
                description: None,
                duration: None,
                name: None,
                parent: None,
                reference: None,
                test_results: None,
            }
        );
    } // it_can_deserialize_with_optional_fields
}
