use serde::{Serialize, Deserialize};
use super::{CreditBankEmployment, CreditBankIncomeAccount};
///The details and metadata for an end user's Item.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreditBankEmploymentItem {
    ///The Item's accounts that have Bank Employment data.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub bank_employment_accounts: Vec<CreditBankIncomeAccount>,
    ///The bank employment information for this Item. Each entry in the array is a different employer found.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub bank_employments: Vec<CreditBankEmployment>,
    ///The unique identifier of the institution associated with the Item.
    pub institution_id: String,
    ///The name of the institution associated with the Item.
    pub institution_name: String,
    ///The unique identifier for the Item.
    pub item_id: String,
    ///The time when this Item's data was last retrieved from the financial institution, in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format (e.g. "2018-04-12T03:32:11Z").
    pub last_updated_time: chrono::DateTime<chrono::Utc>,
}
impl std::fmt::Display for CreditBankEmploymentItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}