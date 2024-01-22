use serde::{Serialize, Deserialize};
///An optional object for `/credit/bank_income/refresh` request options.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreditBankIncomeRefreshRequestOptions {
    ///How many days of data to include in the refresh. If not specified, this will default to the days requested in the most recently generated bank income report for the user.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub days_requested: Option<i64>,
}
impl std::fmt::Display for CreditBankIncomeRefreshRequestOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}