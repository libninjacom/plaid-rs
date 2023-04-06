
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PayStubTaxpayerId {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_mask: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_type: Option<String>,
}
impl std::fmt::Display for PayStubTaxpayerId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}