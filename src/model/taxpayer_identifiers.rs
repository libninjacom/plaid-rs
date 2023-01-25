
use serde::{Serialize, Deserialize};
use super::TaxpayerIdentifier;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaxpayerIdentifiers {
    #[serde(rename = "TAXPAYER_IDENTIFIER")]
    pub taxpayer_identifier: TaxpayerIdentifier,
}
impl std::fmt::Display for TaxpayerIdentifiers {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}