
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Serialize, Deserialize)]
pub enum WatchlistScreeningIndividualUpdateRequestResettableField {
    #[serde(rename = "assignee")]
    Assignee,
}