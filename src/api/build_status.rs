use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum BuildStatusState {
    #[serde(rename = "SUCCESSFUL")]
    Successful,
    #[serde(rename = "FAILED")]
    Failed,
    #[serde(rename = "IN_PROGRESS")]
    InProgress,
    #[serde(rename = "CANCELLED")]
    Cancelled,
    #[serde(rename = "UNKNOWN")]
    Unknown,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TestResults {
    pub failed: u32,
    pub successful: u32,
    pub skipped: u32,
}
