use serde::{Serialize, Deserialize};
use super::{CreditBankIncomeItem, CreditBankIncomeSummary, CreditBankIncomeWarning};
///The report of the Bank Income data for an end user.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreditBankIncome {
    ///The unique identifier associated with the Bank Income Report.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bank_income_id: Option<String>,
    ///Summary for bank income across all income sources and items (max history of 730 days).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bank_income_summary: Option<CreditBankIncomeSummary>,
    ///The number of days requested by the customer for the Bank Income Report.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub days_requested: Option<i64>,
    ///The time when the Bank Income Report was generated.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub generated_time: Option<chrono::DateTime<chrono::Utc>>,
    ///The list of Items in the report along with the associated metadata about the Item.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<CreditBankIncomeItem>>,
    ///If data from the Bank Income report was unable to be retrieved, the warnings will contain information about the error that caused the data to be incomplete.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub warnings: Option<Vec<CreditBankIncomeWarning>>,
}
impl std::fmt::Display for CreditBankIncome {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}