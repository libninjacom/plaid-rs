
use serde::{Serialize, Deserialize};
use super::{Counterparty, Location, PersonalFinanceCategory};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Enrichments {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub check_number: Option<String>,
    pub counterparties: Vec<Counterparty>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub legacy_category: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub legacy_category_id: Option<String>,
    pub location: Location,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logo_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merchant_name: Option<String>,
    pub payment_channel: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub personal_finance_category: Option<PersonalFinanceCategory>,
    pub personal_finance_category_icon_url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub website: Option<String>,
}
impl std::fmt::Display for Enrichments {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}