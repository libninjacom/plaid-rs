use serde::{Serialize, Deserialize};
///An optional set of parameters corresponding to transactions options.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SandboxPublicTokenCreateRequestOptionsTransactions {
    ///The maximum number of days of transaction history to request for the Transactions product.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub days_requested: Option<i64>,
    ///The most recent date for which to fetch transaction history. Dates should be formatted as YYYY-MM-DD.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end_date: Option<chrono::NaiveDate>,
    ///The earliest date for which to fetch transaction history. Dates should be formatted as YYYY-MM-DD.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_date: Option<chrono::NaiveDate>,
}
impl std::fmt::Display for SandboxPublicTokenCreateRequestOptionsTransactions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}