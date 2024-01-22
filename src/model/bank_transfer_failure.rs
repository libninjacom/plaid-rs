use serde::{Serialize, Deserialize};
///The failure reason if the type of this transfer is `"failed"` or `"reversed"`. Null value otherwise.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BankTransferFailure {
    ///The ACH return code, e.g. `R01`.  A return code will be provided if and only if the transfer status is `reversed`. For a full listing of ACH return codes, see [Bank Transfers errors](https://plaid.com/docs/errors/bank-transfers/#ach-return-codes).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ach_return_code: Option<String>,
    ///A human-readable description of the reason for the failure or reversal.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
impl std::fmt::Display for BankTransferFailure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}