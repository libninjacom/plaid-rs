
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DocumentRiskSignal {
    pub actual_value: Option<String>,
    pub expected_value: Option<String>,
    pub field: Option<String>,
    pub has_fraud_risk: Option<bool>,
    pub institution_metadata: Option<DocumentRiskSignalInstitutionMetadata>,
    pub page_number: Option<i64>,
    pub signal_description: Option<String>,
    #[serde(rename = "type")]
    pub type_: Option<String>,
}
impl std::fmt::Display for DocumentRiskSignal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}