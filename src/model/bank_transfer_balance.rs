use serde::{Serialize, Deserialize};
///Information about the balance of a bank transfer
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BankTransferBalance {
    ///The total available balance - the sum of all successful debit transfer amounts minus all credit transfer amounts.
    pub available: String,
    ///The transactable balance shows the amount in your account that you are able to use for transfers, and is essentially your available balance minus your minimum balance.
    pub transactable: String,
}
impl std::fmt::Display for BankTransferBalance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}