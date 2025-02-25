use crate::api::Api;
use crate::client::{ApiRequest, ApiResponse, Client};
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

pub struct PullRequestChangesGetBuilder {
    client: Client,
    project_key: String,
    pull_request_id: String,
    repository_slug: String,
    /// The "since" commit hash to stream changes for a RANGE arbitrary change scope
    since_id: Option<String>,
    /// UNREVIEWED to stream the unreviewed changes for the current user (if they exist); RANGE to stream changes between two arbitrary commits (requires 'sinceId' and 'untilId'); otherwise ALL to stream all changes (the default)
    change_scope: Option<String>,
    /// The "until" commit hash to stream changes for a RANGE arbitrary change scope
    until_id: Option<String>,
    /// Start number for the page (inclusive). If not passed, first page is assumed.
    start: Option<u32>,
    /// Number of items to return. If not passed, a page size of 25 is used.
    limit: Option<u32>,
    /// If true, the response will include all comments on the changed files
    with_comments: Option<bool>,
}

impl PullRequestChangesGetBuilder {
    pub fn since_id(mut self, since_id: &str) -> Self {
        self.since_id = Some(since_id.to_string());
        self
    }
    pub fn change_scope(mut self, change_scope: &str) -> Self {
        self.change_scope = Some(change_scope.to_string());
        self
    }
    pub fn until_id(mut self, until_id: &str) -> Self {
        self.until_id = Some(until_id.to_string());
        self
    }
    pub fn start(mut self, start: u32) -> Self {
        self.start = Some(start);
        self
    }
    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }
    pub fn with_comments(mut self, with_comments: bool) -> Self {
        self.with_comments = Some(with_comments);
        self
    }
}

impl ApiRequest for PullRequestChangesGetBuilder {
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
        PullRequestChangesGetBuilder {
            client: self.client,
            project_key,
            pull_request_id,
            repository_slug,
            since_id: None,
            change_scope: None,
            until_id: None,
            start: None,
            limit: None,
            with_comments: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_can_deserialize() {
        let json = r#"{
            "fromHash":"from_hash",
            "toHash":"to_hash",
            "values":[
                {"contentId":"12345","type":"ADD","path":{"toString":"path/to/file"}},
                {"contentId":"67890","type":"COPY","path":{"toString":"another/target"}}
            ]
        }"#;

        let pull_request_changes: PullRequestChanges = serde_json::from_str(json).unwrap();

        assert_eq!(
            pull_request_changes,
            PullRequestChanges {
                from_hash: "from_hash".to_string(),
                to_hash: "to_hash".to_string(),
                values: Some(vec![
                    ChangeItem {
                        content_id: "12345".to_string(),
                        change_type: "ADD".to_string(),
                        path: Path {
                            to_string: "path/to/file".to_string()
                        }
                    },
                    ChangeItem {
                        content_id: "67890".to_string(),
                        change_type: "COPY".to_string(),
                        path: Path {
                            to_string: "another/target".to_string()
                        }
                    }
                ])
            }
        );
    } // end of it_can_deserialize

    #[test]
    fn it_can_serialize() {
        let pull_request_changes = PullRequestChanges {
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
        };

        let json = serde_json::to_string(&pull_request_changes).unwrap();
        assert_eq!(
            json,
            r#"{"fromHash":"from_hash","toHash":"to_hash","values":[{"contentId":"12345","type":"ADD","path":{"toString":"path/to/file"}},{"contentId":"67890","type":"COPY","path":{"toString":"another/target"}}]}"#
        );
    } // end of it_can_serialize
}
