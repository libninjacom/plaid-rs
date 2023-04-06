
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AddressMatchScore {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_postal_code_match: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score: Option<i64>,
}
impl std::fmt::Display for AddressMatchScore {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}