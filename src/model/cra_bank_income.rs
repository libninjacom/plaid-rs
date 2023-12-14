
use serde::{Serialize, Deserialize};
use super::{CraBankIncomeItem, CraBankIncomeSummary, CraBankIncomeWarning};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CraBankIncome {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_income_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_income_summary: Option<CraBankIncomeSummary>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub days_requested: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generated_time: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<CraBankIncomeItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warnings: Option<Vec<CraBankIncomeWarning>>,
}
impl std::fmt::Display for CraBankIncome {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}