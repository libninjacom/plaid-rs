
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TaxpayerIdentifier {
    #[serde(rename = "TaxpayerIdentifierType")]
    pub taxpayer_identifier_type: Option<String>,
    #[serde(rename = "TaxpayerIdentifierValue")]
    pub taxpayer_identifier_value: Option<String>,
}
impl std::fmt::Display for TaxpayerIdentifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}