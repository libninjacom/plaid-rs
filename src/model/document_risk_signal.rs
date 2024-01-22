use serde::{Serialize, Deserialize};
use super::DocumentRiskSignalInstitutionMetadata;
///Details about a certain reason as to why a document could potentially be fraudulent.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DocumentRiskSignal {
    ///The derived value obtained in the risk signal calculation process for this field
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub actual_value: Option<String>,
    ///The expected value of the field, as seen on the document
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expected_value: Option<String>,
    ///The field which the risk signal was computed for
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub field: Option<String>,
    ///A flag used to quickly identify if the signal indicates that this field is authentic or fraudulent
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_fraud_risk: Option<bool>,
    ///An object which contains additional metadata about the institution used to compute the verification attribute
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub institution_metadata: Option<DocumentRiskSignalInstitutionMetadata>,
    ///The relevant page associated with the risk signal
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_number: Option<i64>,
    ///A human-readable explanation providing more detail into the particular risk signal
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub signal_description: Option<String>,
    ///The result from the risk signal check.
    #[serde(rename = "type")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl std::fmt::Display for DocumentRiskSignal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}