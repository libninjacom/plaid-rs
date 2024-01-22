use serde::{Serialize, Deserialize};
///Information about the Taxpayer identification values assigned to the individual or legal entity.Information about the Taxpayer identification values assigned to the individual or legal entity.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TaxpayerIdentifier {
    ///A value from a MISMO prescribed list that classifies identification numbers used by the Internal Revenue Service (IRS) in the administration of tax laws. A Social Security number (SSN) is issued by the SSA; all other taxpayer identification numbers are issued by the IRS.
    #[serde(rename = "TaxpayerIdentifierType")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub taxpayer_identifier_type: Option<String>,
    ///The value of the taxpayer identifier as assigned by the IRS to the individual or legal entity.
    #[serde(rename = "TaxpayerIdentifierValue")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub taxpayer_identifier_value: Option<String>,
}
impl std::fmt::Display for TaxpayerIdentifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}