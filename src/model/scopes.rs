
use serde::{Serialize, Deserialize};
use super::{AccountAccess, ProductAccess};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Scopes {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accounts: Option<Vec<AccountAccess>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_accounts: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_access: Option<ProductAccess>,
}
impl std::fmt::Display for Scopes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}