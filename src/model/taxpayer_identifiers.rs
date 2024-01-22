use serde::{Serialize, Deserialize};
use super::TaxpayerIdentifier;
///The collection of TAXPAYER_IDENTIFICATION elements
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TaxpayerIdentifiers {
    ///Information about the Taxpayer identification values assigned to the individual or legal entity.Information about the Taxpayer identification values assigned to the individual or legal entity.
    #[serde(rename = "TAXPAYER_IDENTIFIER")]
    pub taxpayer_identifier: TaxpayerIdentifier,
}
impl std::fmt::Display for TaxpayerIdentifiers {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}