use crate::api::build_status::{BuildStatusState, TestResults};
use crate::api::Api;
use crate::client::{ApiRequest, ApiResponse, Client};
use chrono::{serde::ts_seconds_option, DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// The build status associated with the provided commit and key.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BuildStatus {
    pub key: String,
    pub state: BuildStatusState,
    pub url: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub build_number: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none", with = "ts_seconds_option")]
    pub updated_date: Option<DateTime<Utc>>,

    #[serde(skip_serializing_if = "Option::is_none", with = "ts_seconds_option")]
    pub created_date: Option<DateTime<Utc>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent: Option<String>,

    #[serde(rename = "ref", skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub test_results: Option<TestResults>,
}

#[derive(Debug)]
pub struct BuildStatusGetBuilder {
    client: Client,
    project_key: String,
    commit_id: String,
    repository_slug: String,
    key: Option<String>,
}

impl BuildStatusGetBuilder {
    /// the key of the build status
    pub fn key(mut self, key: &str) -> Self {
        self.key = Some(key.to_string());
        self
    }
}

impl ApiRequest for BuildStatusGetBuilder {
    type Output = BuildStatus;

    async fn send(&self) -> ApiResponse<Self::Output> {
        let request_uri = format!(
            "api/latest/projects/{}/repos/{}/commits/{}/builds",
            self.project_key, self.repository_slug, self.commit_id
        );

        let mut params = HashMap::new();

        if let Some(key) = &self.key {
            params.insert("key".to_string(), key.clone());
        }

        self.client
            .get::<BuildStatusGetBuilder>(&request_uri, Some(params))
            .await
    }
}

impl Api {
    /// Get a specific build status.
    ///
    /// See the [Bitbucket Data Center REST API documentation](https://developer.atlassian.com/server/bitbucket/rest/v905/api-group-builds-and-deployments/#api-api-latest-projects-projectkey-repos-repositoryslug-commits-commitid-builds-get).
    ///
    /// # Arguments
    /// * `project_key` - the key of the project
    /// * `commit_id` - the commit id
    /// * `repository_slug` - the slug of the repository
    pub fn build_status_get(
        self,
        project_key: String,
        commit_id: String,
        repository_slug: String,
    ) -> BuildStatusGetBuilder {
        BuildStatusGetBuilder {
            client: self.client,
            project_key,
            commit_id,
            repository_slug,
            key: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::from_str;

    #[test]
    fn it_can_deserialize() {
        let json = r#"{
            "key": "KEY",
            "state": "SUCCESSFUL",
            "url": "https://my-build-status.com/path",
            "buildNumber": "9",
            "createdDate": 1738198923,
            "updatedDate": 1738198924,
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
        }"#;

        let build_status: BuildStatus = from_str(json).unwrap();

        assert_eq!(build_status.key, "KEY");
        assert_eq!(build_status.state, BuildStatusState::Successful);
        assert_eq!(build_status.url, "https://my-build-status.com/path");
        assert_eq!(build_status.build_number.unwrap(), "9");
        assert_eq!(build_status.created_date.unwrap().timestamp(), 1738198923);
        assert_eq!(build_status.updated_date.unwrap().timestamp(), 1738198924);
        assert_eq!(build_status.duration.unwrap(), 12);
        assert_eq!(build_status.description.unwrap(), "DESCRIPTION");
        assert_eq!(build_status.name.unwrap(), "NAME");
        assert_eq!(build_status.parent.unwrap(), "PARENT");
        assert_eq!(build_status.reference.unwrap(), "REF");
        assert_eq!(
            build_status.test_results.unwrap(),
            TestResults {
                failed: 2,
                successful: 3,
                skipped: 1
            }
        );
    }
}
