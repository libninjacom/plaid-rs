
use serde::{Serialize, Deserialize};
use super::PaystubAddress;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PaystubEmployer {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<PaystubAddress>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl std::fmt::Display for PaystubEmployer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}