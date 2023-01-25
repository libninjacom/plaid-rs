
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Serialize, Deserialize)]
pub enum ProgramNameSensitivity {
    #[serde(rename = "coarse")]
    Coarse,
    #[serde(rename = "balanced")]
    Balanced,
    #[serde(rename = "strict")]
    Strict,
    #[serde(rename = "exact")]
    Exact,
}