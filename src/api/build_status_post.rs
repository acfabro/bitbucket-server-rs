use crate::api::build_status::{BuildStatusState, TestResults};
use crate::api::Api;
use crate::client::{ApiRequest, ApiResponse, Client};
use chrono::{serde::ts_seconds_option, DateTime, Utc};
use serde::{Deserialize, Serialize};

/// The POST request payload
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BuildStatusPostPayload {
    /// The string referring to this branch plan/job
    pub key: String,
    /// The build status state
    pub state: BuildStatusState,
    /// URL referring to the build result page in the CI tool.
    pub url: String,
    /// A unique identifier for this particular run of a plan
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build_number: Option<String>,
    /// Date added
    #[serde(skip_serializing_if = "Option::is_none", with = "ts_seconds_option")]
    pub date_added: Option<DateTime<Utc>>,
    /// Describes the build result
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Duration of a completed build
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<u64>,
    /// A short string that describes the build plan
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The identifier for the plan or job that ran the branch plan that produced this build status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent: Option<String>,
    /// The fully qualified git reference e.g. refs/heads/master.
    #[serde(skip_serializing_if = "Option::is_none", rename = "ref")]
    pub reference: Option<String>,
    /// A summary of the passed, failed and skipped tests.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test_results: Option<TestResults>,
}

#[derive(Debug)]
pub struct BuildStatusPost {
    client: Client,
    project_key: String,
    commit_id: String,
    repository_slug: String,
    build_status: BuildStatusPostPayload,
}

impl ApiRequest for BuildStatusPost {
    // response has no content
    type Output = ();

    async fn send(&self) -> ApiResponse<Self::Output> {
        let request_uri = format!(
            "api/latest/projects/{}/repos/{}/commits/{}/builds",
            self.project_key, self.repository_slug, self.commit_id
        );

        self.client
            .post::<Self>(
                &request_uri,
                &serde_json::to_string(&self.build_status).unwrap(),
            )
            .await
    }
}

impl Api {
    /// Store a build status
    ///
    /// Notes:
    /// * `build_status.state` is initially set to `UNKNOWN`. Use the `state_*` methods to set the
    /// state before calling `send()`.
    /// * The authenticated user must have REPO_READ permission for the repository that this build
    /// status is for. The request can also be made with anonymous 2-legged OAuth.
    ///
    /// See [Bitbucket Data Center REST API Docs](https://developer.atlassian.com/server/bitbucket/rest/v811/api-group-builds-and-deployments/#api-api-latest-projects-projectkey-repos-repositoryslug-commits-commitid-builds-post)
    pub fn build_status_post(
        self,
        project_key: &str,
        commit_id: &str,
        repository_slug: &str,
        build_status: &BuildStatusPostPayload,
    ) -> BuildStatusPost {
        BuildStatusPost {
            client: self.client,
            project_key: project_key.to_owned(),
            commit_id: commit_id.to_owned(),
            repository_slug: repository_slug.to_owned(),
            build_status: build_status.to_owned(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_can_serialize() {
        let build_status = BuildStatusPostPayload {
            key: "KEY".to_string(),
            state: BuildStatusState::Successful,
            url: "URL".to_string(),
            build_number: Some("1".to_string()),
            date_added: Some(
                chrono::DateTime::parse_from_rfc3339("2025-01-30T01:02:03Z")
                    .unwrap()
                    .with_timezone(&Utc),
            ),
            description: Some("DESCRIPTION".to_string()),
            duration: Some(12),
            name: Some("NAME".to_string()),
            parent: Some("PARENT".to_string()),
            reference: Some("REF".to_string()),
            test_results: Some(TestResults {
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
    fn it_can_serialize_partially() {
        let build_status = BuildStatusPostPayload {
            key: "KEY".to_string(),
            state: BuildStatusState::Successful,
            url: "URL".to_string(),
            build_number: None,
            date_added: None,
            description: None,
            duration: None,
            name: None,
            parent: None,
            reference: None,
            test_results: None,
        };

        let json = serde_json::to_string(&build_status).unwrap();
        assert_eq!(json, r#"{"key":"KEY","state":"SUCCESSFUL","url":"URL"}"#);
    } // it_can_serialize_partially
}
