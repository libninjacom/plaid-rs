
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Serialize, Deserialize)]
pub enum AchClass {
    #[serde(rename = "ccd")]
    Ccd,
    #[serde(rename = "ppd")]
    Ppd,
    #[serde(rename = "tel")]
    Tel,
    #[serde(rename = "web")]
    Web,
}