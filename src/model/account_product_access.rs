
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AccountProductAccess {
    pub account_data: Option<bool>,
    pub statements: Option<bool>,
    pub tax_documents: Option<bool>,
}
impl std::fmt::Display for AccountProductAccess {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}