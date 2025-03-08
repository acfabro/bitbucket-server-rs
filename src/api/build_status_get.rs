//! # Build Status GET API
//!
//! This module provides functionality to retrieve build status information from Bitbucket Server.
//! It allows fetching the status of builds for specific commits, which can be used to
//! integrate CI/CD systems with Bitbucket Server.

use crate::api::build_status::{BuildStatusState, TestResults};
use crate::api::Api;
use crate::client::{ApiRequest, ApiResponse, Client};
use chrono::{serde::ts_seconds_option, DateTime, Utc};
use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Represents the build status associated with a commit.
///
/// This struct contains information about a build status, including its state,
/// URL, and other metadata.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BuildStatus {
    /// The string referring to this branch plan/job.
    ///
    /// This is a unique identifier for the build status within the context of the repository.
    pub key: String,
    
    /// The build status state (SUCCESSFUL, FAILED, INPROGRESS, CANCELLED, or UNKNOWN).
    pub state: BuildStatusState,
    
    /// URL referring to the build result page in the CI tool.
    ///
    /// This URL is linked from the Bitbucket Server UI.
    pub url: String,

    /// A unique identifier for this particular run of a plan.
    ///
    /// This can be used to track specific build runs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build_number: Option<String>,

    /// The date when the build status was last updated.
    #[serde(skip_serializing_if = "Option::is_none", with = "ts_seconds_option")]
    pub updated_date: Option<DateTime<Utc>>,

    /// The date when the build status was created.
    #[serde(skip_serializing_if = "Option::is_none", with = "ts_seconds_option")]
    pub created_date: Option<DateTime<Utc>>,

    /// A description of the build result.
    ///
    /// This provides additional context about the build status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Duration of a completed build in milliseconds.
    ///
    /// This can be used to track build performance.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<u64>,

    /// A short string that describes the build plan.
    ///
    /// This is displayed in the Bitbucket Server UI.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// The identifier for the plan or job that ran the branch plan that produced this build status.
    ///
    /// This can be used to group related builds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent: Option<String>,

    /// The fully qualified git reference e.g. refs/heads/master.
    ///
    /// This associates the build status with a specific branch or tag.
    #[serde(rename = "ref", skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,

    /// A summary of the passed, failed and skipped tests.
    ///
    /// This provides test result information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test_results: Option<TestResults>,
}

/// Request builder for retrieving build status information.
///
/// This struct is used to build and send requests to retrieve build status information.
#[derive(Debug, Default, Builder)]
pub struct BuildStatusGet {
    /// The HTTP client to use for making requests
    pub client: Client,
    
    /// The key of the project containing the repository
    pub project_key: String,
    
    /// The ID of the commit to get the build status for
    pub commit_id: String,
    
    /// The slug of the repository
    pub repository_slug: String,
    
    /// Optional key to filter build statuses by
    #[builder(setter(into, strip_option), default)]
    pub key: Option<String>,
}

impl ApiRequest for BuildStatusGet {
    type Output = BuildStatus;

    /// Sends the request to retrieve build status information.
    ///
    /// # Returns
    ///
    /// A Result containing either the build status or an error.
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
    /// Creates a request builder for retrieving build status information.
    ///
    /// This method returns a builder that can be used to configure and send a request
    /// to retrieve build status information for a commit.
    ///
    /// # Arguments
    ///
    /// * `project_key` - The key of the project containing the repository
    /// * `commit_id` - The ID of the commit to get the build status for
    /// * `repository_slug` - The slug of the repository
    ///
    /// # Returns
    ///
    /// A builder for configuring and sending the request
    ///
    /// # Example
    ///
    /// ```no_run
    /// use bitbucket_server_rs::client::{new, ApiRequest};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = new("https://bitbucket-server/rest", "API_TOKEN");
    ///
    ///     // Get build status for a specific commit
    ///     let response = client
    ///         .api()
    ///         .build_status_get(
    ///             "PROJECT_KEY",
    ///             "COMMIT_ID",
    ///             "REPOSITORY_SLUG"
    ///         )
    ///         .key("build-123") // Optional: filter by build key
    ///         .build()?
    ///         .send()
    ///         .await?;
    ///
    ///     // Handle the response
    ///     match response {
    ///         Some(build_status) => {
    ///             println!("Build state: {:?}", build_status.state);
    ///             println!("Build URL: {}", build_status.url);
    ///         },
    ///         None => println!("No build status found")
    ///     }
    ///
    ///     Ok(())
    /// }
    /// ```
    ///
    /// See the [Bitbucket Data Center REST API documentation](https://developer.atlassian.com/server/bitbucket/rest/v905/api-group-builds-and-deployments/#api-api-latest-projects-projectkey-repos-repositoryslug-commits-commitid-builds-get)
    pub fn build_status_get(
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
