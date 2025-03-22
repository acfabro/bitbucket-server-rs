//! # Pull Request POST API
//!
//! This module provides functionality to create pull requests in Bitbucket Server.
//! It allows creating pull requests between branches with customizable titles,
//! descriptions, and reviewers.

use crate::api::Api;
use crate::client::{ApiRequest, ApiResponse, Client};
use serde::{Deserialize, Serialize};

/// A user or group that can be added as a reviewer to a pull request
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Reviewer {
    /// The user or group ID
    pub user: User,
}

/// A user in Bitbucket Server
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct User {
    /// The name of the user
    pub name: String,
}

/// The payload for creating a pull request.
///
/// This struct represents the data that will be sent to the Bitbucket Server API
/// when creating a new pull request.
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PullRequestPostPayload {
    /// The title of the pull request
    pub title: String,

    /// The description of the pull request
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// The branch information for the pull request
    pub from_ref: RefInfo,

    /// The target branch information for the pull request
    pub to_ref: RefInfo,

    /// The list of reviewers for the pull request
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reviewers: Option<Vec<Reviewer>>,
}

/// Information about a Git reference (branch)
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RefInfo {
    /// The ID of the reference (usually branch name)
    pub id: String,

    /// The repository the branch belongs to
    pub repository: RepositoryInfo,
}

/// Information about a repository
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RepositoryInfo {
    /// The slug of the repository
    pub slug: String,

    /// The project the repository belongs to
    pub project: ProjectInfo,
}

/// Information about a project
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectInfo {
    /// The key of the project
    pub key: String,
}

/// Request builder for creating a pull request.
///
/// This struct is used to build and send requests to create pull requests.
#[derive(Debug)]
pub struct PullRequestPost {
    /// The HTTP client to use for making requests
    client: Client,
    
    /// The key of the project containing the repository
    project_key: String,
    
    /// The slug of the repository
    repository_slug: String,
    
    /// The pull request payload to post
    pull_request: PullRequestPostPayload,
}

impl ApiRequest for PullRequestPost {
    type Output = PullRequestPostPayload;

    /// Sends the request to create a pull request.
    ///
    /// # Returns
    ///
    /// A Result containing the created pull request information or an error.
    async fn send(&self) -> ApiResponse<Self::Output> {
        let request_uri = format!(
            "api/latest/projects/{}/repos/{}/pull-requests",
            self.project_key, self.repository_slug
        );

        self.client
            .post::<Self>(
                &request_uri,
                &serde_json::to_string(&self.pull_request).unwrap(),
            )
            .await
    }
}

impl Api {
    /// Creates a request to create a new pull request.
    ///
    /// This method returns a builder that can be used to send a request
    /// to create a new pull request.
    ///
    /// # Arguments
    ///
    /// * `project_key` - The key of the project containing the repository
    /// * `repository_slug` - The slug of the repository
    /// * `pull_request` - The pull request payload
    ///
    /// # Returns
    ///
    /// A builder for sending the request
    ///
    /// # Example
    ///
    /// ```no_run
    /// use bitbucket_server_rs::client::{new, ApiRequest};
    /// use bitbucket_server_rs::api::pull_request_post::{
    ///     PullRequestPostPayload, RefInfo, RepositoryInfo, ProjectInfo
    /// };
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = new("https://bitbucket-server/rest", "API_TOKEN");
    ///
    ///     // Create pull request payload
    ///     let pull_request = PullRequestPostPayload {
    ///         title: "Add new feature".to_string(),
    ///         description: Some("Implements the new feature".to_string()),
    ///         from_ref: RefInfo {
    ///             id: "refs/heads/feature-branch".to_string(),
    ///             repository: RepositoryInfo {
    ///                 slug: "my-repo".to_string(),
    ///                 project: ProjectInfo {
    ///                     key: "PROJECT".to_string(),
    ///                 },
    ///             },
    ///         },
    ///         to_ref: RefInfo {
    ///             id: "refs/heads/main".to_string(),
    ///             repository: RepositoryInfo {
    ///                 slug: "my-repo".to_string(),
    ///                 project: ProjectInfo {
    ///                     key: "PROJECT".to_string(),
    ///                 },
    ///             },
    ///         },
    ///         reviewers: None,
    ///     };
    ///
    ///     // Create the pull request
    ///     let response = client
    ///         .api()
    ///         .pull_request_post(
    ///             "PROJECT",
    ///             "my-repo",
    ///             &pull_request
    ///         )
    ///         .send()
    ///         .await?;
    ///
    ///     println!("Pull request created successfully");
    ///
    ///     Ok(())
    /// }
    /// ```
    ///
    /// # Notes
    ///
    /// * The authenticated user must have REPO_WRITE permission for the repository to create pull requests.
    ///
    /// See [Bitbucket Data Center REST API Docs](https://developer.atlassian.com/server/bitbucket/rest/v811/api-group-pull-requests/#api-api-latest-projects-projectkey-repos-repositoryslug-pull-requests-post)
    pub fn pull_request_post(
        self,
        project_key: &str,
        repository_slug: &str,
        pull_request: &PullRequestPostPayload,
    ) -> PullRequestPost {
        PullRequestPost {
            client: self.client,
            project_key: project_key.to_owned(),
            repository_slug: repository_slug.to_owned(),
            pull_request: pull_request.to_owned(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_can_serialize() {
        let pull_request = PullRequestPostPayload {
            title: "Test PR".to_string(),
            description: Some("Test description".to_string()),
            from_ref: RefInfo {
                id: "refs/heads/feature".to_string(),
                repository: RepositoryInfo {
                    slug: "test-repo".to_string(),
                    project: ProjectInfo {
                        key: "TEST".to_string(),
                    },
                },
            },
            to_ref: RefInfo {
                id: "refs/heads/main".to_string(),
                repository: RepositoryInfo {
                    slug: "test-repo".to_string(),
                    project: ProjectInfo {
                        key: "TEST".to_string(),
                    },
                },
            },
            reviewers: Some(vec![Reviewer {
                user: User {
                    name: "testuser".to_string(),
                },
            }]),
        };

        let json = serde_json::to_string(&pull_request).unwrap();
        assert_eq!(
            json,
            r#"{"title":"Test PR","description":"Test description","fromRef":{"id":"refs/heads/feature","repository":{"slug":"test-repo","project":{"key":"TEST"}}},"toRef":{"id":"refs/heads/main","repository":{"slug":"test-repo","project":{"key":"TEST"}}},"reviewers":[{"user":{"name":"testuser"}}]}"#
        );
    }

    #[test]
    fn it_can_serialize_partially() {
        let pull_request = PullRequestPostPayload {
            title: "Test PR".to_string(),
            description: None,
            from_ref: RefInfo {
                id: "refs/heads/feature".to_string(),
                repository: RepositoryInfo {
                    slug: "test-repo".to_string(),
                    project: ProjectInfo {
                        key: "TEST".to_string(),
                    },
                },
            },
            to_ref: RefInfo {
                id: "refs/heads/main".to_string(),
                repository: RepositoryInfo {
                    slug: "test-repo".to_string(),
                    project: ProjectInfo {
                        key: "TEST".to_string(),
                    },
                },
            },
            reviewers: None,
        };

        let json = serde_json::to_string(&pull_request).unwrap();
        assert_eq!(
            json,
            r#"{"title":"Test PR","fromRef":{"id":"refs/heads/feature","repository":{"slug":"test-repo","project":{"key":"TEST"}}},"toRef":{"id":"refs/heads/main","repository":{"slug":"test-repo","project":{"key":"TEST"}}}}"#
        );
    }
}
