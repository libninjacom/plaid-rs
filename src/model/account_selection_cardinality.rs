
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Serialize, Deserialize)]
pub enum AccountSelectionCardinality {
    #[serde(rename = "SINGLE_SELECT")]
    SingleSelect,
    #[serde(rename = "MULTI_SELECT")]
    MultiSelect,
    #[serde(rename = "ALL")]
    All,
}