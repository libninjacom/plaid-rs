
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TaxpayerIdentifier {
    #[serde(rename = "TaxpayerIdentifierType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub taxpayer_identifier_type: Option<String>,
    #[serde(rename = "TaxpayerIdentifierValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub taxpayer_identifier_value: Option<String>,
}
impl std::fmt::Display for TaxpayerIdentifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}