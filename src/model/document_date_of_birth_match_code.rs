
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Serialize, Deserialize)]
pub enum DocumentDateOfBirthMatchCode {
    #[serde(rename = "match")]
    Match,
    #[serde(rename = "partial_match")]
    PartialMatch,
    #[serde(rename = "no_match")]
    NoMatch,
}