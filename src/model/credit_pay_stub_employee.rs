
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreditPayStubEmployee {
    pub address: CreditPayStubAddress,
    pub marital_status: Option<String>,
    pub name: Option<String>,
    pub taxpayer_id: PayStubTaxpayerId,
}
impl std::fmt::Display for CreditPayStubEmployee {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}