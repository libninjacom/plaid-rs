use serde::{Serialize, Deserialize};
use super::{AccountAccess, ProductAccess};
///The scopes object
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Scopes {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub accounts: Option<Vec<AccountAccess>>,
    ///Allow access to newly opened accounts as they are opened. If unset, defaults to `true`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub new_accounts: Option<bool>,
    ///The product access being requested. Used to or disallow product access across all accounts. If unset, defaults to all products allowed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub product_access: Option<ProductAccess>,
}
impl std::fmt::Display for Scopes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}