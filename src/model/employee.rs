use serde::{Serialize, Deserialize};
use super::{PaystubAddress, TaxpayerId};
///Data about the employee.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Employee {
    ///Address on the paystub
    pub address: PaystubAddress,
    ///Marital status of the employee - either `single` or `married`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub marital_status: Option<String>,
    ///The name of the employee.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    ///Taxpayer ID of the individual receiving the paystub.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub taxpayer_id: Option<TaxpayerId>,
}
impl std::fmt::Display for Employee {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}