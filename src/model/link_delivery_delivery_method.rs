
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Serialize, Deserialize)]
pub enum LinkDeliveryDeliveryMethod {
    #[serde(rename = "SMS")]
    Sms,
    #[serde(rename = "EMAIL")]
    Email,
}