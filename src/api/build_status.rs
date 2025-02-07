use crate::api::Api;
use chrono::{serde::ts_seconds_option, DateTime, Utc};
use log::debug;
use reqwest::Method;
use serde::{Deserialize, Serialize};
use serde_json::json;

///
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BuildStatus {
    key: String,
    state: BuildStatusState,
    url: String,

    #[serde(rename = "buildNumber", skip_serializing_if = "Option::is_none")]
    build_number: Option<String>,

    #[serde(
        rename = "dateAdded",
        with = "ts_seconds_option",
        skip_serializing_if = "Option::is_none",
        default
    )]
    date_added: Option<DateTime<Utc>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    duration: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    parent: Option<String>,

    #[serde(rename = "ref", skip_serializing_if = "Option::is_none")]
    reference: Option<String>,

    #[serde(rename = "testResults", skip_serializing_if = "Option::is_none")]
    test_results: Option<TestResults>,
}

impl BuildStatus {
    pub fn new(key: String, state: BuildStatusState, url: String) -> BuildStatus {
        BuildStatus {
            key,
            state,
            url,
            build_number: None,
            date_added: None,
            description: None,
            duration: None,
            name: None,
            parent: None,
            reference: None,
            test_results: None,
        }
    }

    pub fn with_build_number(mut self, build_number: String) -> BuildStatus {
        self.build_number = Some(build_number);
        self
    }

    pub fn with_date_added(mut self, date_added: DateTime<Utc>) -> BuildStatus {
        self.date_added = Some(date_added);
        self
    }

    pub fn with_description(mut self, description: String) -> BuildStatus {
        self.description = Some(description);
        self
    }

    pub fn with_duration_secs(mut self, duration: u64) -> BuildStatus {
        self.duration = Some(duration);
        self
    }

    pub fn with_name(mut self, name: String) -> BuildStatus {
        self.name = Some(name);
        self
    }

    pub fn with_parent(mut self, parent: String) -> BuildStatus {
        self.parent = Some(parent);
        self
    }

    pub fn with_reference(mut self, reference: String) -> BuildStatus {
        self.reference = Some(reference);
        self
    }

    pub fn with_test_results(mut self, successful: u32, failed: u32, skipped: u32) -> BuildStatus {
        self.test_results = Some(TestResults {
            successful,
            failed,
            skipped,
        });
        self
    }

    pub fn with_state_successful(mut self) -> BuildStatus {
        self.state = BuildStatusState::Successful;
        self
    }
    pub fn with_state_failed(mut self) -> BuildStatus {
        self.state = BuildStatusState::Failed;
        self
    }
    pub fn with_state_in_progress(mut self) -> BuildStatus {
        self.state = BuildStatusState::InProgress;
        self
    }
    pub fn with_state_cancelled(mut self) -> BuildStatus {
        self.state = BuildStatusState::Cancelled;
        self
    }
    pub fn with_state_unknown(mut self) -> BuildStatus {
        self.state = BuildStatusState::Unknown;
        self
    }
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
struct TestResults {
    failed: u32,
    successful: u32,
    skipped: u32,
}

#[derive(Clone, Debug)]
pub struct PostBuildStatusParams {
    project_key: String,
    commit_id: String,
    repository_slug: String,
    build_status: BuildStatus,
}

impl PostBuildStatusParams {
    pub fn new(
        project_key: String,
        commit_id: String,
        repository_slug: String,
        build_status: BuildStatus,
    ) -> PostBuildStatusParams {
        PostBuildStatusParams {
            project_key,
            commit_id,
            repository_slug,
            build_status,
        }
    }
}

impl Api<'_> {
    pub async fn post_build_status(&self, params: PostBuildStatusParams) -> Result<(), String> {
        let build_status = params.build_status;
        let api_client = self.client;
        let http_client = &api_client.client;

        let request_uri = format!(
            "{}/api/v1/repos/{}/{}/commits/{}/build-status",
            api_client.base_path, params.project_key, params.repository_slug, params.commit_id
        );
        let request_builder = http_client
            .request(Method::POST, request_uri.as_str())
            .header("Content-Type", "application/json");

        debug!("Got build status: {}", json!(build_status));
        let request = request_builder.json(&build_status).build().unwrap();
        let response = http_client.execute(request).await.unwrap();

        if response.status().is_success() {
            Ok(())
        } else {
            Err(format!(
                "Error Response [{}]: {}",
                response.status(),
                response.text().await.unwrap()
            ))
        }
    }

    pub async fn get_build_status(
        &self,
        project_key: String,
        commit_id: String,
        repository_slug: String,
    ) -> Result<BuildStatus, String> {
        let http_client = &self.client.client;
        let request_uri = format!(
            "{}/api/v1/repos/{}/{}/commits/{}/build-status",
            self.client.base_path, project_key, repository_slug, commit_id
        );

        let response = http_client
            .get(&request_uri)
            .header("Authorization", format!("Bearer {}", self.client.api_token))
            .header("Content-Type", "application/json")
            .send();

        let response = response.await.unwrap();
        let status = response.status();
        let body = response.text().await.unwrap();

        if status.is_success() {
            let build_status: BuildStatus = serde_json::from_str(body.as_str()).unwrap();
            Ok(build_status)
        } else {
            Err(format!("Error Response [{}]: {}", status, body))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::BuildStatus;
    use serde_json::from_str;

    #[test]
    fn it_can_serialize() {
        let build_status = BuildStatus::new(
            "KEY".to_string(),
            super::BuildStatusState::Successful,
            "URL".to_string(),
        )
        .with_build_number("1".to_string())
        .with_date_added(
            chrono::DateTime::parse_from_rfc3339("2025-01-30T01:02:03Z")
                .unwrap()
                .with_timezone(&chrono::Utc),
        )
        .with_description("DESCRIPTION".to_string())
        .with_duration_secs(12)
        .with_name("NAME".to_string())
        .with_parent("PARENT".to_string())
        .with_reference("REF".to_string())
        .with_test_results(3, 2, 1);

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

        let build_status: BuildStatus = from_str(json).unwrap();

        assert_eq!(
            build_status,
            BuildStatus {
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
          "url": "https://my-bitbucket-server.com/browse/TEST-REP3"
        }"#;

        let build_status: BuildStatus = from_str(json).unwrap();
        assert_eq!(
            build_status,
            BuildStatus {
                key: "TEST-REP3".to_string(),
                state: super::BuildStatusState::Successful,
                url: "https://my-bitbucket-server.com/browse/TEST-REP3".to_string(),
                build_number: None,
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
