
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Serialize, Deserialize)]
pub enum ScopesContext {
    #[serde(rename = "ENROLLMENT")]
    Enrollment,
    #[serde(rename = "PORTAL")]
    Portal,
}