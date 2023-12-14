
use serde::{Serialize, Deserialize};
use super::{CraBankIncomeAccount, CraBankIncomeSource};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CraBankIncomeItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_income_accounts: Option<Vec<CraBankIncomeAccount>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_income_sources: Option<Vec<CraBankIncomeSource>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub institution_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub institution_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_time: Option<chrono::DateTime<chrono::Utc>>,
}
impl std::fmt::Display for CraBankIncomeItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}