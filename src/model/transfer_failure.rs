use serde::{Serialize, Deserialize};
///The failure reason if the event type for a transfer is `"failed"` or `"returned"`. Null value otherwise.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TransferFailure {
    ///The ACH return code, e.g. `R01`.  A return code will be provided if and only if the transfer status is `returned`. For a full listing of ACH return codes, see [Transfer errors](https://plaid.com/docs/errors/transfer/#ach-return-codes).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ach_return_code: Option<String>,
    ///A human-readable description of the reason for the failure or reversal.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
impl std::fmt::Display for TransferFailure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}