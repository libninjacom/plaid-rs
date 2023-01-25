
use serde::{Serialize, Deserialize};
use super::{Counterparty, Location, PersonalFinanceCategory};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Enrichments {
    pub check_number: Option<String>,
    pub counterparties: Vec<Counterparty>,
    pub legacy_category: Option<Vec<String>>,
    pub legacy_category_id: Option<String>,
    pub location: Location,
    pub logo_url: Option<String>,
    pub merchant_name: Option<String>,
    pub payment_channel: String,
    pub personal_finance_category: Option<PersonalFinanceCategory>,
    pub personal_finance_category_icon_url: String,
    pub website: Option<String>,
}
impl std::fmt::Display for Enrichments {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}