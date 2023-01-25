
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PartnerCustomerOAuthInstitutionsGetRequest(pub serde_json::Value);