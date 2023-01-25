
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Enhancements {
    pub category: Vec<String>,
    pub category_id: Option<String>,
    pub check_number: Option<String>,
    pub counterparties: Option<Vec<Counterparty>>,
    pub location: Location,
    pub logo_url: Option<String>,
    pub merchant_name: Option<String>,
    pub payment_channel: String,
    pub personal_finance_category: Option<PersonalFinanceCategory>,
    pub personal_finance_category_icon_url: Option<String>,
    pub website: Option<String>,
}
impl std::fmt::Display for Enhancements {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}