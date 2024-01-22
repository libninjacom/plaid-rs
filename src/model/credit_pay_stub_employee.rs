use serde::{Serialize, Deserialize};
use super::{CreditPayStubAddress, PayStubTaxpayerId};
///Data about the employee.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreditPayStubEmployee {
    ///Address on the pay stub.
    pub address: CreditPayStubAddress,
    ///Marital status of the employee - either `SINGLE` or `MARRIED` or `NOT LISTED`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub marital_status: Option<String>,
    ///The name of the employee.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    ///Taxpayer ID of the individual receiving the paystub.
    pub taxpayer_id: PayStubTaxpayerId,
}
impl std::fmt::Display for CreditPayStubEmployee {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}