
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Serialize, Deserialize)]
pub enum ReportType {
    #[serde(rename = "assets")]
    Assets,
    #[serde(rename = "income")]
    Income,
}