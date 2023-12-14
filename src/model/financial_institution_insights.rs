
use serde::{Serialize, Deserialize};
use super::DetectedAccount;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FinancialInstitutionInsights {
    pub detected_accounts: Vec<DetectedAccount>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_id: Option<String>,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub website: Option<String>,
}
impl std::fmt::Display for FinancialInstitutionInsights {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}