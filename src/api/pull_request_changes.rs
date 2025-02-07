use crate::api::Api;
use serde::{Deserialize, Serialize};

/// This module is responsible for handling the pull request changes API.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PullRequestChange {
    #[serde(rename = "fromHash")]
    pub from_hash: String,
    #[serde(rename = "toHash")]
    pub to_hash: String,
    /// Array of changes
    #[serde(rename = "values", skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<Change>>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Change {
    #[serde(rename = "contentId")]
    pub content_id: String,
    #[serde(rename = "type")]
    pub change_type: String,
    #[serde(rename = "path")]
    pub path: Path,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Path {
    #[serde(rename = "toString")]
    pub to_string: String,
}

impl Api<'_> {
    pub async fn get_pull_request_changes(
        &self,
        project_key: String,
        pull_request_id: String,
        repository_slug: String,
    ) -> Result<PullRequestChange, String> {
        let http_client = &self.client.client;
        let request_uri = format!(
            "{}/api/latest/projects/{}/repos/{}/pull-requests/{}/changes",
            self.client.base_path, project_key, repository_slug, pull_request_id
        );

        let response = http_client
            .get(request_uri.as_str())
            .header("Authorization", format!("Bearer {}", self.client.api_token))
            .header("Content-Type", "application/json")
            .send();

        let response = response.await.unwrap();
        let status = response.status();
        let body = response.text().await.unwrap();

        if status.is_success() {
            let pull_request_changes: PullRequestChange =
                serde_json::from_str(body.as_str()).unwrap();
            Ok(pull_request_changes)
        } else {
            Err(format!("Error Response [{}]: {}", status, body))
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

        let pull_request_changes: PullRequestChange = serde_json::from_str(json).unwrap();

        assert_eq!(
            pull_request_changes,
            PullRequestChange {
                from_hash: "from_hash".to_string(),
                to_hash: "to_hash".to_string(),
                values: Some(vec![
                    Change {
                        content_id: "12345".to_string(),
                        change_type: "ADD".to_string(),
                        path: Path {
                            to_string: "path/to/file".to_string()
                        }
                    },
                    Change {
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
        let pull_request_changes = PullRequestChange {
            from_hash: "from_hash".to_string(),
            to_hash: "to_hash".to_string(),
            values: Some(vec![
                Change {
                    content_id: "12345".to_string(),
                    change_type: "ADD".to_string(),
                    path: Path {
                        to_string: "path/to/file".to_string(),
                    },
                },
                Change {
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
