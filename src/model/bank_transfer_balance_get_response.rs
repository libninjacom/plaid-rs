
use serde::{Serialize, Deserialize};
use super::BankTransferBalance;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BankTransferBalanceGetResponse {
    pub balance: BankTransferBalance,
    pub origination_account_id: Option<String>,
    pub request_id: String,
}
impl std::fmt::Display for BankTransferBalanceGetResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}