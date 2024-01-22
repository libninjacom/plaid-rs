use serde::{Serialize, Deserialize};
use super::{CraBankIncomeAccount, CraBankIncomeSource};
///The details and metadata for an end user's Item.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CraBankIncomeItem {
    ///The Item's accounts that have Bank Income data.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bank_income_accounts: Option<Vec<CraBankIncomeAccount>>,
    ///The income sources for this Item. Each entry in the array is a single income source.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bank_income_sources: Option<Vec<CraBankIncomeSource>>,
    ///The unique identifier of the institution associated with the Item.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub institution_id: Option<String>,
    ///The name of the institution associated with the Item.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub institution_name: Option<String>,
    ///The time when this Item's data was last retrieved from the financial institution.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_updated_time: Option<chrono::DateTime<chrono::Utc>>,
}
impl std::fmt::Display for CraBankIncomeItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}