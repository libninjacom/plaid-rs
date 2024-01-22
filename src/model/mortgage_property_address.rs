use serde::{Serialize, Deserialize};
///Object containing fields describing property address.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MortgagePropertyAddress {
    ///The city name.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    ///The ISO 3166-1 alpha-2 country code.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    ///The five or nine digit postal code.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
    ///The region or state (example "NC").
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    ///The full street address (example "564 Main Street, Apt 15").
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub street: Option<String>,
}
impl std::fmt::Display for MortgagePropertyAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}