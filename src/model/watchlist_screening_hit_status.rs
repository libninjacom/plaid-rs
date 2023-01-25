
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Serialize, Deserialize)]
pub enum WatchlistScreeningHitStatus {
    #[serde(rename = "confirmed")]
    Confirmed,
    #[serde(rename = "pending_review")]
    PendingReview,
    #[serde(rename = "dismissed")]
    Dismissed,
}