use serde::{Serialize, Deserialize};
///The address associated with the account holder.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TransferUserAddressInResponse {
    ///Ex. "San Francisco"
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    ///A two-letter country code (e.g., "US").
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    ///The postal code (e.g., "94103").
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
    ///The state or province (e.g., "CA").
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    ///The street number and name (i.e., "100 Market St.").
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub street: Option<String>,
}
impl std::fmt::Display for TransferUserAddressInResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}