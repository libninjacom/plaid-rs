use serde::{Serialize, Deserialize};
use super::DetectedAccount;
///Insights surrounding external financial institution counterparties associated with a user.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FinancialInstitutionInsights {
    ///Associated accounts, detected based on the nature of transfers to/from this institution.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub detected_accounts: Vec<DetectedAccount>,
    ///A unique, stable, Plaid-generated id that maps to the counterparty.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub entity_id: Option<String>,
    ///Name of the financial institution counterparty.
    pub name: String,
    ///The website associated with the counterparty.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub website: Option<String>,
}
impl std::fmt::Display for FinancialInstitutionInsights {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}