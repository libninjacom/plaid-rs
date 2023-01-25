
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PartnerCustomerEnableRequest(pub serde_json::Value);