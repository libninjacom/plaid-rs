
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Serialize, Deserialize)]
pub enum WatchlistScreeningEntityUpdateRequestResettableField {
    #[serde(rename = "assignee")]
    Assignee,
}