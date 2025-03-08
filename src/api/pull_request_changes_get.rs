//! # Pull Request Changes API
//!
//! This module provides functionality to retrieve changes in pull requests from Bitbucket Server.
//! It allows fetching the list of files that were modified, added, or deleted in a pull request.

use crate::api::Api;
use crate::client::{ApiRequest, ApiResponse, Client};
use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Represents the changes in a pull request.
///
/// This struct contains information about the changes between the source and target branches
/// of a pull request, including the list of modified files.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PullRequestChanges {
    /// The commit hash of the source branch
    pub from_hash: String,
    
    /// The commit hash of the target branch
    pub to_hash: String,
    
    /// Array of changes (files that were modified, added, or deleted)
    #[serde(rename = "values", skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<ChangeItem>>,
}

/// Represents a single change item in a pull request.
///
/// This struct contains information about a single file that was changed in a pull request.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChangeItem {
    /// The content ID of the change
    #[serde(rename = "contentId")]
    pub content_id: String,
    
    /// The type of change (e.g., "ADD", "MODIFY", "DELETE")
    #[serde(rename = "type")]
    pub change_type: String,
    
    /// The path of the file that was changed
    #[serde(rename = "path")]
    pub path: Path,
}

/// Represents the path of a file in a change.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Path {
    /// The string representation of the path
    #[serde(rename = "toString")]
    pub to_string: String,
}

/// Request builder for retrieving pull request changes.
///
/// This struct is used to build and send requests to retrieve changes in a pull request.
#[derive(Debug, Default, Builder)]
pub struct PullRequestChangesGet {
    /// The HTTP client to use for making requests
    client: Client,
    
    /// The key of the project containing the repository
    project_key: String,
    
    /// The ID of the pull request
    pull_request_id: String,
    
    /// The slug of the repository
    repository_slug: String,
    
    /// The "since" commit hash to stream changes for a RANGE arbitrary change scope
    #[builder(setter(into, strip_option), default)]
    since_id: Option<String>,
    
    /// UNREVIEWED to stream the unreviewed changes for the current user (if they exist);
    /// RANGE to stream changes between two arbitrary commits (requires 'sinceId' and 'untilId');
    /// otherwise ALL to stream all changes (the default)
    #[builder(setter(into, strip_option), default)]
    change_scope: Option<String>,
    
    /// The "until" commit hash to stream changes for a RANGE arbitrary change scope
    #[builder(setter(into, strip_option), default)]
    until_id: Option<String>,
    
    /// Start number for the page (inclusive). If not passed, first page is assumed.
    #[builder(setter(into, strip_option), default)]
    start: Option<u32>,
    
    /// Number of items to return. If not passed, a page size of 25 is used.
    #[builder(setter(into, strip_option), default)]
    limit: Option<u32>,
    
    /// If true, the response will include all comments on the changed files
    #[builder(setter(into, strip_option), default)]
    with_comments: Option<bool>,
}

impl ApiRequest for PullRequestChangesGet {
    type Output = PullRequestChanges;

    /// Sends the request to retrieve pull request changes.
    ///
    /// # Returns
    ///
    /// A Result containing either the pull request changes or an error.
    async fn send(&self) -> ApiResponse<Self::Output> {
        let request_uri = format!(
            "api/latest/projects/{}/repos/{}/pull-requests/{}/changes",
            self.project_key, self.repository_slug, self.pull_request_id
        );

        let mut params = HashMap::new();

        if let Some(since_id) = &self.since_id {
            params.insert("sinceId".to_string(), since_id.clone());
        }
        if let Some(change_scope) = &self.change_scope {
            params.insert("changeScope".to_string(), change_scope.clone());
        }
        if let Some(until_id) = &self.until_id {
            params.insert("untilId".to_string(), until_id.clone());
        }
        if let Some(start) = &self.start {
            params.insert("start".to_string(), start.to_string());
        }
        if let Some(limit) = &self.limit {
            params.insert("limit".to_string(), limit.to_string());
        }
        if let Some(with_comments) = &self.with_comments {
            params.insert("withComments".to_string(), with_comments.to_string());
        }

        self.client.get::<Self>(&request_uri, Some(params)).await
    }
}

impl Api {
    /// Creates a request builder for retrieving changes in a pull request.
    ///
    /// This method returns a builder that can be used to configure and send a request
    /// to retrieve the changes in a pull request.
    ///
    /// # Arguments
    ///
    /// * `project_key` - The key of the project containing the repository
    /// * `repository_slug` - The slug of the repository
    /// * `pull_request_id` - The ID of the pull request
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
    ///     let response = client
    ///         .api()
    ///         .pull_request_changes_get("PROJECT", "REPO", "123")
    ///         .limit(50)
    ///         .build()?
    ///         .send()
    ///         .await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    ///
    /// See the [Bitbucket Data Center REST API documentation](https://developer.atlassian.com/server/bitbucket/rest/v811/api-group-pull-requests/#api-api-latest-projects-projectkey-repos-repositoryslug-pull-requests-pullrequestid-changes-get)
    pub fn pull_request_changes_get(
        self,
        project_key: &str,
        repository_slug: &str,
        pull_request_id: &str,
    ) -> PullRequestChangesGetBuilder {
        let mut builder = PullRequestChangesGetBuilder::default();
        builder
            .client(self.client.clone())
            .project_key(project_key.to_string())
            .repository_slug(repository_slug.to_string())
            .pull_request_id(pull_request_id.to_string());
        builder
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_can_deserialize() {
        let json = mock_json();
        let pull_request_changes: PullRequestChanges = serde_json::from_str(&json).unwrap();

        assert_eq!(pull_request_changes, mock_struct());
    } // end of it_can_deserialize

    #[test]
    fn it_can_serialize() {
        let pull_request_changes_struct = mock_struct();
        let json = serde_json::to_string(&pull_request_changes_struct).unwrap();

        assert_eq!(json, mock_json());
    } // end of it_can_serialize

    fn mock_struct() -> PullRequestChanges {
        PullRequestChanges {
            from_hash: "from_hash".to_string(),
            to_hash: "to_hash".to_string(),
            values: Some(vec![
                ChangeItem {
                    content_id: "12345".to_string(),
                    change_type: "ADD".to_string(),
                    path: Path {
                        to_string: "path/to/file".to_string(),
                    },
                },
                ChangeItem {
                    content_id: "67890".to_string(),
                    change_type: "COPY".to_string(),
                    path: Path {
                        to_string: "another/target".to_string(),
                    },
                },
            ]),
        }
    }

    fn mock_json() -> String {
        r#"{"fromHash":"from_hash","toHash":"to_hash","values":[{"contentId":"12345","type":"ADD","path":{"toString":"path/to/file"}},{"contentId":"67890","type":"COPY","path":{"toString":"another/target"}}]}"#.to_string()
    }
}
