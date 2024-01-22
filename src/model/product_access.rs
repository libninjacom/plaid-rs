use serde::{Serialize, Deserialize};
///The product access being requested. Used to or disallow product access across all accounts. If unset, defaults to all products allowed.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ProductAccess {
    ///Allow access to "accounts_details_transactions". Only used by certain partners. If relevant to the partner and unset, defaults to `true`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub accounts_details_transactions: Option<bool>,
    ///Allow access to "accounts_routing_number". Only used by certain partners. If relevant to the partner and unset, defaults to `true`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub accounts_routing_number: Option<bool>,
    ///Allow access to "accounts_statements". Only used by certain partners. If relevant to the partner and unset, defaults to `true`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub accounts_statements: Option<bool>,
    ///Allow access to "accounts_tax_statements". Only used by certain partners. If relevant to the partner and unset, defaults to `true`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub accounts_tax_statements: Option<bool>,
    ///Allow access to account number details. Only used by certain partners. If relevant to the partner and unset, defaults to `true`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub auth: Option<bool>,
    ///Allow access to "customers_profiles". Only used by certain partners. If relevant to the partner and unset, defaults to `true`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub customers_profiles: Option<bool>,
    ///Allow access to the Identity product (name, email, phone, address). Only used by certain partners. If relevant to the partner and unset, defaults to `true`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<bool>,
    ///Allow access to statements. Only used by certain partners. If relevant to the partner and unset, defaults to `true`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub statements: Option<bool>,
    ///Allow access to transaction details. Only used by certain partners. If relevant to the partner and unset, defaults to `true`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transactions: Option<bool>,
}
impl std::fmt::Display for ProductAccess {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}