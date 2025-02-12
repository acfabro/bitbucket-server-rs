use crate::api::Api;
use crate::client::Client;
use reqwest::{Method, StatusCode};
use serde::{Deserialize, Serialize};

/// This module is responsible for handling the pull request changes API.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PullRequestChanges {
    #[serde(rename = "fromHash")]
    from_hash: String,
    #[serde(rename = "toHash")]
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

pub struct PullRequestChangesBuilder {
    client: Client,
    project_key: String,
    pull_request_id: String,
    repository_slug: String,
    path: String,

    // todo: add other fields
}

impl PullRequestChangesBuilder {
    pub async fn send(&self) -> Result<PullRequestChanges, String> {
        let api_client = self;

        let request_uri = format!(
            "{}/api/latest/projects/{}/repos/{}/pull-requests/{}/changes?path={}",
            api_client.client.base_path,
            self.project_key,
            self.repository_slug,
            self.pull_request_id,
            self.path,
        );

        let http_client = &api_client.client.client;
        let request_builder = http_client
            .request(Method::GET, request_uri.as_str())
            .header("Content-Type", "application/json");

        let request = request_builder.build().unwrap();
        let response = http_client.execute(request).await.unwrap();

        match response.status() {
            //
            StatusCode::OK => {
                let pull_request_changes = Self::handle_response(response).await;
                pull_request_changes
            },

            //
            _ => Err(format!(
                "Unexpected Response [{}]: {}",
                response.status(),
                response.text().await.unwrap()
            )),
        }
    }

    async fn handle_response(response: reqwest::Response) -> Result<PullRequestChanges, String> {
        let status = response.status();
        let body = response.text().await.unwrap();

        if status.is_success() {
            let pull_request_changes: PullRequestChanges =
                serde_json::from_str(body.as_str()).unwrap();
            Ok(pull_request_changes)
        } else {
            Err(format!("Error Response [{}]: {}", status, body))
        }
    }
}

impl Api {
    pub fn get_pull_request_changes(
        self,
        project_key: String,
        pull_request_id: String,
        repository_slug: String,
        path: String,
    ) -> PullRequestChangesBuilder {
        PullRequestChangesBuilder {
            client: self.client,
            project_key,
            pull_request_id,
            repository_slug,
            path,
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
