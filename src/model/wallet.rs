use serde::{Serialize, Deserialize};
use super::{WalletBalance, WalletNumbers};
///An object representing the e-wallet
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Wallet {
    ///An object representing the e-wallet balance
    pub balance: WalletBalance,
    ///An object representing the e-wallet account numbers
    pub numbers: WalletNumbers,
    ///The ID of the recipient that corresponds to the e-wallet account numbers
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recipient_id: Option<String>,
    /**The status of the wallet.

`UNKNOWN`: The wallet status is unknown.

`ACTIVE`: The wallet is active and ready to send money to and receive money from.

`CLOSED`: The wallet is closed. Any transactions made to or from this wallet will error.*/
    pub status: String,
    ///A unique ID identifying the e-wallet
    pub wallet_id: String,
}
impl std::fmt::Display for Wallet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}