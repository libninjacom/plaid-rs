
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Serialize, Deserialize)]
pub enum TransferScheduleIntervalUnit {
    #[serde(rename = "week")]
    Week,
    #[serde(rename = "month")]
    Month,
}