
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Serialize, Deserialize)]
pub enum IssuingCountry {
    #[serde(rename = "match")]
    Match,
    #[serde(rename = "no_match")]
    NoMatch,
}