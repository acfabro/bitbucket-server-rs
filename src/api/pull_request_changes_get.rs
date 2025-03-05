use crate::api::Api;
use crate::client::{ApiRequest, ApiResponse, Client};
use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// This module is responsible for handling the pull request changes API.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PullRequestChanges {
    from_hash: String,
    to_hash: String,
    /// Array of changes
    #[serde(rename = "values", skip_serializing_if = "Option::is_none")]
    values: Option<Vec<ChangeItem>>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
struct ChangeItem {
    #[serde(rename = "contentId")]
    content_id: String,
    #[serde(rename = "type")]
    change_type: String,
    #[serde(rename = "path")]
    path: Path,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
struct Path {
    #[serde(rename = "toString")]
    to_string: String,
}

#[derive(Debug, Default, Builder)]
pub struct PullRequestChangesGet {
    client: Client,
    project_key: String,
    pull_request_id: String,
    repository_slug: String,
    /// The "since" commit hash to stream changes for a RANGE arbitrary change scope
    #[builder(setter(into, strip_option), default)]
    since_id: Option<String>,
    /// UNREVIEWED to stream the unreviewed changes for the current user (if they exist); RANGE to stream changes between two arbitrary commits (requires 'sinceId' and 'untilId'); otherwise ALL to stream all changes (the default)
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
    pub fn pull_request_changes_get(
        self,
        project_key: String,
        repository_slug: String,
        pull_request_id: String,
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

