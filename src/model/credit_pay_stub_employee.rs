
use serde::{Serialize, Deserialize};
use super::{CreditPayStubAddress, PayStubTaxpayerId};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreditPayStubEmployee {
    pub address: CreditPayStubAddress,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marital_status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    pub taxpayer_id: PayStubTaxpayerId,
}
impl std::fmt::Display for CreditPayStubEmployee {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}