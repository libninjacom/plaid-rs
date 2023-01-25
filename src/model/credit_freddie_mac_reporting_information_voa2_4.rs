
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreditFreddieMacReportingInformationVoa24 {
    #[serde(rename = "ReportDateTime")]
    pub report_date_time: Option<String>,
    #[serde(rename = "ReportIdentifierType")]
    pub report_identifier_type: Option<String>,
    #[serde(rename = "ReportingInformationIdentifier")]
    pub reporting_information_identifier: String,
}
impl std::fmt::Display for CreditFreddieMacReportingInformationVoa24 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}