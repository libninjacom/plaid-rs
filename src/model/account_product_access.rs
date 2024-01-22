use serde::{Serialize, Deserialize};
///Allow the application to access specific products on this account
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AccountProductAccess {
    ///Allow the application to access account data. Only used by certain partners. If relevant to the partner and unset, defaults to `true`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub account_data: Option<bool>,
    ///Allow the application to access bank statements. Only used by certain partners. If relevant to the partner and unset, defaults to `true`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub statements: Option<bool>,
    ///Allow the application to access tax documents. Only used by certain partners. If relevant to the partner and unset, defaults to `true`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tax_documents: Option<bool>,
}
impl std::fmt::Display for AccountProductAccess {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}