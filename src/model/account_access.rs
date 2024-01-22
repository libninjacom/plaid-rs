use serde::{Serialize, Deserialize};
use super::AccountProductAccess;
///Allow or disallow product access by account. Unlisted (e.g. missing) accounts will be considered `new_accounts`.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AccountAccess {
    ///Allow the application to access specific products on this account
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub account_product_access: Option<AccountProductAccess>,
    ///Allow the application to see this account (and associated details, including balance) in the list of accounts  If unset, defaults to `true`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub authorized: Option<bool>,
    ///The unique account identifier for this account. This value must match that returned by the data access API for this account.
    pub unique_id: String,
}
impl std::fmt::Display for AccountAccess {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}