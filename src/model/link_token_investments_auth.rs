
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LinkTokenInvestmentsAuth {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manual_entry_enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub masked_number_match_enabled: Option<bool>,
}
impl std::fmt::Display for LinkTokenInvestmentsAuth {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}