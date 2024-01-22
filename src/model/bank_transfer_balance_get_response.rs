use serde::{Serialize, Deserialize};
use super::BankTransferBalance;
///Defines the response schema for `/bank_transfer/balance/get`
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BankTransferBalanceGetResponse {
    ///Information about the balance of a bank transfer
    pub balance: BankTransferBalance,
    ///The ID of the origination account that this balance belongs to.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origination_account_id: Option<String>,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
}
impl std::fmt::Display for BankTransferBalanceGetResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}