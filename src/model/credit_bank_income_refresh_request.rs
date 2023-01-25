
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreditBankIncomeRefreshRequest {
    pub options: Option<CreditBankIncomeRefreshRequestOptions>,
    pub user_token: String,
}
impl std::fmt::Display for CreditBankIncomeRefreshRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}