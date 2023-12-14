
use serde::{Serialize, Deserialize};
use super::{PaymentInitiationAddress, WalletTransactionCounterpartyNumbers};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WalletTransactionCounterparty {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<PaymentInitiationAddress>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_of_birth: Option<chrono::NaiveDate>,
    pub name: String,
    pub numbers: WalletTransactionCounterpartyNumbers,
}
impl std::fmt::Display for WalletTransactionCounterparty {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}