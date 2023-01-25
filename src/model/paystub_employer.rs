
use serde::{Serialize, Deserialize};
use super::PaystubAddress;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PaystubEmployer {
    pub address: Option<PaystubAddress>,
    pub name: Option<String>,
}
impl std::fmt::Display for PaystubEmployer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}