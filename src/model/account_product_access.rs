
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AccountProductAccess {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_data: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statements: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_documents: Option<bool>,
}
impl std::fmt::Display for AccountProductAccess {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}