
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Serialize, Deserialize)]
pub enum ServiceProductFulfillmentIdentifier {
    #[serde(rename = "VOA")]
    Voa,
    #[serde(rename = "VOETRANSACTIONS")]
    Voetransactions,
}