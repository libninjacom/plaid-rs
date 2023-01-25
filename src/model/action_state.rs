
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Serialize, Deserialize)]
pub enum ActionState {
    #[serde(rename = "UNKNOWN")]
    Unknown,
    #[serde(rename = "ATTEMPT")]
    Attempt,
    #[serde(rename = "SUCCESS")]
    Success,
    #[serde(rename = "FAILURE")]
    Failure,
    #[serde(rename = "SKIPPED")]
    Skipped,
}