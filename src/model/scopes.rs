
use serde::{Serialize, Deserialize};
use super::{AccountAccess, ProductAccess};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Scopes {
    pub accounts: Option<Vec<AccountAccess>>,
    pub new_accounts: Option<bool>,
    pub product_access: Option<ProductAccess>,
}
impl std::fmt::Display for Scopes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}