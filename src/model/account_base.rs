
use serde::{Serialize, Deserialize};
use super::{AccountBalance, AccountSubtype};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountBase {
    pub account_id: String,
    pub balances: AccountBalance,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mask: Option<String>,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub official_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub persistent_account_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subtype: Option<AccountSubtype>,
    #[serde(rename = "type")]
    pub type_: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_status: Option<String>,
}
impl std::fmt::Display for AccountBase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}