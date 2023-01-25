
use serde::{Serialize, Deserialize};
use super::{AccountBalance, AccountSubtype};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountBase {
    pub account_id: String,
    pub balances: AccountBalance,
    pub mask: Option<String>,
    pub name: String,
    pub official_name: Option<String>,
    pub subtype: Option<AccountSubtype>,
    #[serde(rename = "type")]
    pub type_: String,
    pub verification_status: Option<String>,
}
impl std::fmt::Display for AccountBase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}