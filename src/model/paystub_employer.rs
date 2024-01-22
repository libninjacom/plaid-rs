use serde::{Serialize, Deserialize};
use super::PaystubAddress;
///Information about the employer on the paystub
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PaystubEmployer {
    ///Address on the paystub
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<PaystubAddress>,
    ///The name of the employer on the paystub.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl std::fmt::Display for PaystubEmployer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}