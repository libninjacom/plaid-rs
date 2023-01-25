
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreditBankIncomeHistoricalSummary {
    pub end_date: Option<String>,
    pub iso_currency_code: Option<String>,
    pub start_date: Option<String>,
    pub total_amount: Option<f64>,
    pub total_amounts: Option<Vec<CreditAmountWithCurrency>>,
    pub transactions: Option<Vec<CreditBankIncomeTransaction>>,
    pub unofficial_currency_code: Option<String>,
}
impl std::fmt::Display for CreditBankIncomeHistoricalSummary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}