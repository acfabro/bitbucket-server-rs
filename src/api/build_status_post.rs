//! # Build Status POST API
//!
//! This module provides functionality to post build status updates to Bitbucket Server.
//! It allows setting the status of a build for a specific commit, which can be used
//! to integrate CI/CD systems with Bitbucket Server.

use crate::api::build_status::{BuildStatusState, TestResults};
use crate::api::Api;
use crate::client::{ApiRequest, ApiResponse, Client};
use chrono::{serde::ts_seconds_option, DateTime, Utc};
use serde::{Deserialize, Serialize};

/// The payload for posting a build status update.
///
/// This struct represents the data that will be sent to the Bitbucket Server API
/// when posting a build status update for a commit.
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BuildStatusPostPayload {
    /// The string referring to this branch plan/job.
    ///
    /// This is a unique identifier for the build status within the context of the repository.
    pub key: String,
    
    /// The build status state (SUCCESSFUL, FAILED, INPROGRESS, CANCELLED, or UNKNOWN).
    pub state: BuildStatusState,
    
    /// URL referring to the build result page in the CI tool.
    ///
    /// This URL will be linked from the Bitbucket Server UI.
    pub url: String,
    
    /// A unique identifier for this particular run of a plan.
    ///
    /// This can be used to track specific build runs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build_number: Option<String>,
    
    /// The date when the build status was added.
    ///
    /// If not provided, the current time will be used.
    #[serde(skip_serializing_if = "Option::is_none", with = "ts_seconds_option")]
    pub date_added: Option<DateTime<Utc>>,
    
    /// A description of the build result.
    ///
    /// This can provide additional context about the build status.
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
    /// This can be used to associate the build status with a specific branch or tag.
    #[serde(skip_serializing_if = "Option::is_none", rename = "ref")]
    pub reference: Option<String>,
    
    /// A summary of the passed, failed and skipped tests.
    ///
    /// This can be used to provide test result information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test_results: Option<TestResults>,
}

/// Request builder for posting a build status update.
///
/// This struct is used to build and send requests to post build status updates.
#[derive(Debug)]
pub struct BuildStatusPost {
    /// The HTTP client to use for making requests
    client: Client,
    
    /// The key of the project containing the repository
    project_key: String,
    
    /// The ID of the commit to post the build status for
    commit_id: String,
    
    /// The slug of the repository
    repository_slug: String,
    
    /// The build status payload to post
    build_status: BuildStatusPostPayload,
}

impl ApiRequest for BuildStatusPost {
    // response has no content
    type Output = ();

    /// Sends the request to post a build status update.
    ///
    /// # Returns
    ///
    /// A Result indicating success or failure.
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
    /// Creates a request to post a build status update for a commit.
    ///
    /// This method returns a builder that can be used to send a request
    /// to post a build status update for a commit.
    ///
    /// # Arguments
    ///
    /// * `project_key` - The key of the project containing the repository
    /// * `repository_slug` - The slug of the repository
    /// * `commit_id` - The ID of the commit to post the build status for
    /// * `build_status` - The build status payload to post
    ///
    /// # Returns
    ///
    /// A builder for sending the request
    ///
    /// # Example
    ///
    /// ```no_run
    /// use bitbucket_server_rs::client::{new, ApiRequest};
    /// use bitbucket_server_rs::api::build_status::BuildStatusState;
    /// use bitbucket_server_rs::api::build_status_post::BuildStatusPostPayload;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = new("https://bitbucket-server/rest", "API_TOKEN");
    ///
    ///     // Create build status payload
    ///     let build_status = BuildStatusPostPayload {
    ///         key: "build-123".to_string(),
    ///         state: BuildStatusState::Successful,
    ///         url: "https://ci.example.com/build/123".to_string(),
    ///         description: Some("Build passed successfully".to_string()),
    ///         name: Some("CI Build".to_string()),
    ///         ..Default::default()
    ///     };
    ///
    ///     // Post the build status
    ///     let response = client
    ///         .api()
    ///         .build_status_post(
    ///             "PROJECT_KEY",
    ///             "REPOSITORY_SLUG",
    ///             "COMMIT_ID",
    ///             &build_status
    ///         )
    ///         .send()
    ///         .await?;
    ///
    ///     println!("Build status posted successfully");
    ///
    ///     Ok(())
    /// }
    /// ```
    ///
    /// # Notes
    ///
    /// * The authenticated user must have REPO_READ permission for the repository that this build
    ///   status is for. The request can also be made with anonymous 2-legged OAuth.
    ///
    /// See [Bitbucket Data Center REST API Docs](https://developer.atlassian.com/server/bitbucket/rest/v811/api-group-builds-and-deployments/#api-api-latest-projects-projectkey-repos-repositoryslug-commits-commitid-builds-post)
    pub fn build_status_post(
        self,
        project_key: &str,
        repository_slug: &str,
        commit_id: &str,
        build_status: &BuildStatusPostPayload
    ) -> BuildStatusPost {
        BuildStatusPost {
            client: self.client,
            project_key: project_key.to_owned(),
            commit_id: commit_id.to_owned(),
            repository_slug: repository_slug.to_owned(),
            build_status: build_status.to_owned()
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
