
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TaxpayerId {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_mask: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_type: Option<String>,
    #[serde(rename = "last_4_digits")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last4_digits: Option<String>,
}
impl std::fmt::Display for TaxpayerId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}