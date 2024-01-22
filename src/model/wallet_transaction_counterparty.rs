use serde::{Serialize, Deserialize};
use super::{PaymentInitiationAddress, WalletTransactionCounterpartyNumbers};
///An object representing the e-wallet transaction's counterparty
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WalletTransactionCounterparty {
    ///The optional address of the payment recipient's bank account. Required by most institutions outside of the UK.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<PaymentInitiationAddress>,
    ///The counterparty's birthdate, in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) (YYYY-MM-DD) format.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub date_of_birth: Option<chrono::NaiveDate>,
    ///The name of the counterparty
    pub name: String,
    ///The counterparty's bank account numbers. Exactly one of IBAN or BACS data is required.
    pub numbers: WalletTransactionCounterpartyNumbers,
}
impl std::fmt::Display for WalletTransactionCounterparty {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}