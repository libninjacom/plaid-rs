
use serde::{Serialize, Deserialize};
use super::{CraBankIncomeTransaction, CreditAmountWithCurrency};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CraBankIncomeHistoricalSummary {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<chrono::NaiveDate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<chrono::NaiveDate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_amounts: Option<Vec<CreditAmountWithCurrency>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transactions: Option<Vec<CraBankIncomeTransaction>>,
}
impl std::fmt::Display for CraBankIncomeHistoricalSummary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}