
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Serialize, Deserialize)]
pub enum Strategy {
    #[serde(rename = "reset")]
    Reset,
    #[serde(rename = "incomplete")]
    Incomplete,
    #[serde(rename = "infer")]
    Infer,
    #[serde(rename = "custom")]
    Custom,
}