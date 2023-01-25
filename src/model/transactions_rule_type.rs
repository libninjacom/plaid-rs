
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Serialize, Deserialize)]
pub enum TransactionsRuleType {
    #[serde(rename = "EXACT_MATCH")]
    ExactMatch,
    #[serde(rename = "SUBSTRING_MATCH")]
    SubstringMatch,
}