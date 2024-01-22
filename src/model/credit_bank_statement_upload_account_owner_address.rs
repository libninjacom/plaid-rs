use serde::{Serialize, Deserialize};
///Address on the uploaded bank statement
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreditBankStatementUploadAccountOwnerAddress {
    ///The full city name.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    ///The ISO 3166-1 alpha-2 country code.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    ///The postal code of the address.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
    /**The region or state.
Example: `"NC"`*/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    ///The full street address.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub street: Option<String>,
}
impl std::fmt::Display for CreditBankStatementUploadAccountOwnerAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}