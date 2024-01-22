use serde::{Serialize, Deserialize};
///An object containing data on the overall period of the statement.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreditBankStatementUploadBankAccountPeriod {
    ///The end date of the statement period in ISO 8601 format (YYYY-MM-DD).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end_date: Option<chrono::NaiveDate>,
    ///The ending balance of the bank account for the period.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ending_balance: Option<f64>,
    ///The start date of the statement period in ISO 8601 format (YYYY-MM-DD).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_date: Option<chrono::NaiveDate>,
    ///The starting balance of the bank account for the period.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub starting_balance: Option<f64>,
}
impl std::fmt::Display for CreditBankStatementUploadBankAccountPeriod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}