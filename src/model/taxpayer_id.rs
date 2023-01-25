
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TaxpayerId {
    pub id_mask: Option<String>,
    pub id_type: Option<String>,
    #[serde(rename = "last_4_digits")]
    pub last4_digits: Option<String>,
}
impl std::fmt::Display for TaxpayerId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}