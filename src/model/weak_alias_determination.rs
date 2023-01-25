
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Serialize, Deserialize)]
pub enum WeakAliasDetermination {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "source")]
    Source,
    #[serde(rename = "plaid")]
    Plaid,
}