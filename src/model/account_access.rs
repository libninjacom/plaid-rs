
use serde::{Serialize, Deserialize};
use super::AccountProductAccess;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AccountAccess {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_product_access: Option<AccountProductAccess>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorized: Option<bool>,
    pub unique_id: String,
}
impl std::fmt::Display for AccountAccess {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}