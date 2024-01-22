use serde::{Serialize, Deserialize};
use super::PaystubOverrideEmployeeAddress;
///The employee on the paystub.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PaystubOverrideEmployee {
    ///The address of the employee.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<PaystubOverrideEmployeeAddress>,
    ///The name of the employee.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl std::fmt::Display for PaystubOverrideEmployee {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}