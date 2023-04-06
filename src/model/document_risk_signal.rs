
use serde::{Serialize, Deserialize};
use super::DocumentRiskSignalInstitutionMetadata;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DocumentRiskSignal {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actual_value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_fraud_risk: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub institution_metadata: Option<DocumentRiskSignalInstitutionMetadata>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_number: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signal_description: Option<String>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl std::fmt::Display for DocumentRiskSignal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}