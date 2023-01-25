
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Serialize, Deserialize)]
pub enum PaymentChannel {
    #[serde(rename = "online")]
    Online,
    #[serde(rename = "in store")]
    InStore,
    #[serde(rename = "other")]
    Other,
}