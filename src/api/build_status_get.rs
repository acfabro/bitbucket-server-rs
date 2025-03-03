use crate::api::build_status::{BuildStatusState, TestResults};
use crate::api::Api;
use crate::client::{ApiRequest, ApiResponse, Client};
use chrono::{serde::ts_seconds_option, DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use derive_builder::Builder;

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

#[derive(Debug, Default, Builder)]
pub struct BuildStatusGet {
    pub client: Client,
    pub project_key: String,
    pub commit_id: String,
    pub repository_slug: String,
    #[builder(setter(into, strip_option), default)]
    pub key: Option<String>,
}

impl ApiRequest for BuildStatusGet {
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
            .get::<BuildStatusGet>(&request_uri, Some(params))
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
    pub fn get_build_status(
        &self,
        project_key: &str,
        commit_id: &str,
        repository_slug: &str,
    ) -> BuildStatusGetBuilder {
        let mut builder = BuildStatusGetBuilder::default();
        builder
            .client(self.client.clone())
            .project_key(project_key.to_string())
            .commit_id(commit_id.to_string())
            .repository_slug(repository_slug.to_string());
        builder
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
