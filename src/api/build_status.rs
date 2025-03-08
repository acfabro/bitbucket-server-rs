//! # Build Status Common Types
//!
//! This module contains common types and utilities used by the build status API endpoints.
//! These types are shared between the GET and POST operations for build status.

use serde::{Deserialize, Serialize};

/// Represents the state of a build in Bitbucket Server.
///
/// This enum maps to the build status states supported by Bitbucket Server's API.
/// When serialized, it uses uppercase strings as required by the API.
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub enum BuildStatusState {
    /// The build status is unknown or not set
    #[default]
    #[serde(rename = "UNKNOWN")]
    Unknown,
    
    /// The build completed successfully
    #[serde(rename = "SUCCESSFUL")]
    Successful,
    
    /// The build failed
    #[serde(rename = "FAILED")]
    Failed,
    
    /// The build is currently in progress
    #[serde(rename = "INPROGRESS")]
    InProgress,
    
    /// The build was cancelled
    #[serde(rename = "CANCELLED")]
    Cancelled,
}

impl From<String> for BuildStatusState {
    /// Converts a string to a BuildStatusState.
    ///
    /// # Arguments
    ///
    /// * `value` - The string representation of the build status state
    ///
    /// # Returns
    ///
    /// The corresponding BuildStatusState, or Unknown if the string doesn't match any known state
    fn from(value: String) -> Self {
        match value.as_str() {
            "UNKNOWN" => BuildStatusState::Unknown,
            "SUCCESSFUL" => BuildStatusState::Successful,
            "FAILED" => BuildStatusState::Failed,
            "INPROGRESS" => BuildStatusState::InProgress,
            "CANCELLED" => BuildStatusState::Cancelled,
            _ => BuildStatusState::Unknown,
        }
    }
}

/// Represents test results associated with a build.
///
/// This struct contains counts of test results in different states.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TestResults {
    /// Number of failed tests
    pub failed: u32,
    
    /// Number of successful tests
    pub successful: u32,
    
    /// Number of skipped tests
    pub skipped: u32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_can_convert_string_to_state() {
        let state = BuildStatusState::from("SUCCESSFUL".to_string());
        assert_eq!(state, BuildStatusState::Successful);

        let state = BuildStatusState::from("FAILED".to_string());
        assert_eq!(state, BuildStatusState::Failed);

        let state = BuildStatusState::from("INPROGRESS".to_string());
        assert_eq!(state, BuildStatusState::InProgress);

        let state = BuildStatusState::from("CANCELLED".to_string());
        assert_eq!(state, BuildStatusState::Cancelled);

        let state = BuildStatusState::from("UNKNOWN".to_string());
        assert_eq!(state, BuildStatusState::Unknown);

        let state = BuildStatusState::from("InVaLiD".to_string());
        assert_eq!(state, BuildStatusState::Unknown);
    } // end of it_can_convert_string_to_state

}
