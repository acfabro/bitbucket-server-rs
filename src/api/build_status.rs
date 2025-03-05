use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub enum BuildStatusState {
    #[default]
    #[serde(rename = "UNKNOWN")]
    Unknown,
    #[serde(rename = "SUCCESSFUL")]
    Successful,
    #[serde(rename = "FAILED")]
    Failed,
    #[serde(rename = "IN_PROGRESS")]
    InProgress,
    #[serde(rename = "CANCELLED")]
    Cancelled,
}

impl From<String> for BuildStatusState {
    fn from(value: String) -> Self {
        match value.as_str() {
            "UNKNOWN" => BuildStatusState::Unknown,
            "SUCCESSFUL" => BuildStatusState::Successful,
            "FAILED" => BuildStatusState::Failed,
            "IN_PROGRESS" => BuildStatusState::InProgress,
            "CANCELLED" => BuildStatusState::Cancelled,
            _ => BuildStatusState::Unknown,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TestResults {
    pub failed: u32,
    pub successful: u32,
    pub skipped: u32,
}
