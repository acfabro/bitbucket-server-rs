use crate::api::build_status::{BuildStatusState, TestResults};
use crate::api::Api;
use crate::client::{ApiRequest, ApiResponse, Client};
use chrono::{serde::ts_seconds_option, DateTime, Utc};
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// The POST request payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct BuildStatusPostPayload {
    /// The string referring to this branch plan/job
    key: String,
    /// The build status state
    state: BuildStatusState,
    /// URL referring to the build result page in the CI tool.
    url: String,
    /// A unique identifier for this particular run of a plan
    #[serde(skip_serializing_if = "Option::is_none")]
    build_number: Option<String>,
    ///
    #[serde(skip_serializing_if = "Option::is_none", with = "ts_seconds_option")]
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
    #[serde(skip_serializing_if = "Option::is_none", rename = "ref")]
    reference: Option<String>,
    /// A summary of the passed, failed and skipped tests.
    #[serde(skip_serializing_if = "Option::is_none")]
    test_results: Option<TestResults>,
}

#[derive(Debug, Builder)]
pub struct BuildStatusPost {
    client: Client,
    project_key: String,
    commit_id: String,
    repository_slug: String,
    build_status: BuildStatusPostPayload,
}

/// custom setters for build_status
impl BuildStatusPostBuilder {
    pub fn state_successful(&mut self) -> &mut Self {
        let mut build_status = self.build_status.clone().unwrap();
        build_status.state = BuildStatusState::Successful;
        self.build_status(build_status);
        self
    }
    pub fn state_failed(&mut self) -> &mut Self {
        let mut build_status = self.build_status.clone().unwrap();
        build_status.state = BuildStatusState::Failed;
        self.build_status(build_status);
        self
    }
    pub fn state_in_progress(&mut self) -> &mut Self {
        let mut build_status = self.build_status.clone().unwrap();
        build_status.state = BuildStatusState::InProgress;
        self.build_status(build_status);
        self
    }
    pub fn state_cancelled(&mut self) -> &mut Self {
        let mut build_status = self.build_status.clone().unwrap();
        build_status.state = BuildStatusState::Cancelled;
        self.build_status(build_status);
        self
    }
    pub fn state_unknown(&mut self) -> &mut Self {
        let mut build_status = self.build_status.clone().unwrap();
        build_status.state = BuildStatusState::Unknown;
        self.build_status(build_status);
        self
    }
    pub fn build_number(&mut self, build_number: String) -> &mut Self {
        let mut build_status = self.build_status.clone().unwrap();
        build_status.build_number = Some(build_number);
        self.build_status(build_status);
        self
    }
    pub fn date_added(&mut self, date_added: DateTime<Utc>) -> &mut Self {
        let mut build_status = self.build_status.clone().unwrap();
        build_status.date_added = Some(date_added);
        self.build_status(build_status);
        self
    }
    pub fn description(&mut self, description: String) -> &mut Self {
        let mut build_status = self.build_status.clone().unwrap();
        build_status.description = Some(description);
        self.build_status(build_status);
        self
    }
    pub fn duration_secs(&mut self, duration: u64) -> &mut Self {
        let mut build_status = self.build_status.clone().unwrap();
        build_status.duration = Some(duration);
        self.build_status(build_status);
        self
    }
    pub fn name(&mut self, name: String) -> &mut Self {
        let mut build_status = self.build_status.clone().unwrap();
        build_status.name = Some(name);
        self.build_status(build_status);
        self
    }
    pub fn parent(&mut self, parent: String) -> &mut Self {
        let mut build_status = self.build_status.clone().unwrap();
        build_status.parent = Some(parent);
        self.build_status(build_status);
        self
    }
    pub fn reference(&mut self, reference: String) -> &mut Self {
        let mut build_status = self.build_status.clone().unwrap();
        build_status.reference = Some(reference);
        self.build_status(build_status);
        self
    }
    pub fn test_results(
        &mut self,
        successful: u32,
        failed: u32,
        skipped: u32,
    ) -> &mut Self {
        let mut build_status = self.build_status.clone().unwrap();
        build_status.test_results = Some(TestResults {
            successful,
            failed,
            skipped,
        });
        self.build_status(build_status);
        self
    }
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
    pub fn post_build_status(
        self,
        project_key: String,
        commit_id: String,
        repository_slug: String,
        build_status_key: String,
        build_status_url: String,
    ) -> BuildStatusPostBuilder {
        let mut builder = BuildStatusPostBuilder::default();
        builder
            .client(self.client)
            .project_key(project_key)
            .commit_id(commit_id)
            .repository_slug(repository_slug)
            .build_status(BuildStatusPostPayload {
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
            });

        builder
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
