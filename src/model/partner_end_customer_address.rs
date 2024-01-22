use serde::{Serialize, Deserialize};
///The end customer's address.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PartnerEndCustomerAddress {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    ///ISO-3166-1 alpha-2 country code standard.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub country_code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub street: Option<String>,
}
impl std::fmt::Display for PartnerEndCustomerAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}