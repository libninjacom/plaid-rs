
use serde::{Serialize, Deserialize};
use super::{PaystubAddress, TaxpayerId};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Employee {
    pub address: PaystubAddress,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marital_status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub taxpayer_id: Option<TaxpayerId>,
}
impl std::fmt::Display for Employee {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}