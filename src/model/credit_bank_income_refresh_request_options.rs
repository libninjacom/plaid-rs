
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreditBankIncomeRefreshRequestOptions {
    pub days_requested: Option<i64>,
    pub webhook: Option<String>,
}
impl std::fmt::Display for CreditBankIncomeRefreshRequestOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}