use serde::{Serialize, Deserialize};
///Information about the balance of the ledger held with Plaid.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TransferLedgerBalance {
    ///The amount of this balance available for use (decimal string with two digits of precision e.g. "10.00").
    pub available: String,
    ///The amount of pending funds that are in processing (decimal string with two digits of precision e.g. "10.00").
    pub pending: String,
}
impl std::fmt::Display for TransferLedgerBalance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}