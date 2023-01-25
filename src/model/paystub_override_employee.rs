
use serde::{Serialize, Deserialize};
use super::PaystubOverrideEmployeeAddress;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PaystubOverrideEmployee {
    pub address: Option<PaystubOverrideEmployeeAddress>,
    pub name: Option<String>,
}
impl std::fmt::Display for PaystubOverrideEmployee {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}