use crate::api::build_status_post::BuildStatusState;
use crate::api::Api;
use crate::client::Client;
use chrono::{serde::ts_seconds_option, DateTime, Utc};
use reqwest::{Method, StatusCode};
use serde::{Deserialize, Serialize};

///
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
struct BuildStatus {
    key: String,
    state: BuildStatusState,
    url: String,

    #[serde(rename = "buildNumber", skip_serializing_if = "Option::is_none")]
    build_number: Option<String>,

    #[serde(
        rename = "updatedDate",
        with = "ts_seconds_option",
        skip_serializing_if = "Option::is_none",
        default
    )]
    updated_date: Option<DateTime<Utc>>,

    #[serde(
        rename = "createdDate",
        with = "ts_seconds_option",
        skip_serializing_if = "Option::is_none",
        default
    )]
    created_date: Option<DateTime<Utc>>,

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
    test_results: Option<crate::api::build_status_post::TestResults>,
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
    pub fn key(mut self, key: String) -> Self {
        self.key = Some(key);
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

        let request = request_builder.build().unwrap();
        let response = http_client.execute(request).await.unwrap();

        match response.status() {
            //
            StatusCode::OK => Ok(()),
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
    /// Get a specific build status.
    ///
    /// https://developer.atlassian.com/server/bitbucket/rest/latest/api-group-builds-and-deployments/#api-api-latest-projects-projectkey-repos-repositoryslug-commits-commitid-builds-get
    pub fn get_build_status(
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
            "updatedDate": 1738198923,
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
        assert_eq!(build_status.updated_date.unwrap().timestamp(), 1738198923);
        assert_eq!(build_status.duration.unwrap(), 12);
        assert_eq!(build_status.description.unwrap(), "DESCRIPTION");
        assert_eq!(build_status.name.unwrap(), "NAME");
        assert_eq!(build_status.parent.unwrap(), "PARENT");
        assert_eq!(build_status.reference.unwrap(), "REF");
        assert_eq!(
            build_status.test_results.unwrap(),
            crate::api::build_status_post::TestResults {
                failed: 2,
                successful: 3,
                skipped: 1
            }
        );
    }
}
