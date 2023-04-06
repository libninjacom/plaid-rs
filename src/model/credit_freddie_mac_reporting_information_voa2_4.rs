
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreditFreddieMacReportingInformationVoa24 {
    #[serde(rename = "ReportDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub report_date_time: Option<String>,
    #[serde(rename = "ReportIdentifierType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub report_identifier_type: Option<String>,
    #[serde(rename = "ReportingInformationIdentifier")]
    pub reporting_information_identifier: String,
}
impl std::fmt::Display for CreditFreddieMacReportingInformationVoa24 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}