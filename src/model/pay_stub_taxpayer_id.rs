
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PayStubTaxpayerId {
    pub id_mask: Option<String>,
    pub id_type: Option<String>,
}
impl std::fmt::Display for PayStubTaxpayerId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}