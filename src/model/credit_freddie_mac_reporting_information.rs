use serde::{Serialize, Deserialize};
///Information about an report identifier and a report name.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreditFreddieMacReportingInformation {
    ///Documentation not found in the MISMO model viewer and not provided by Freddie Mac.
    #[serde(rename = "ReportDateTime")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub report_date_time: Option<String>,
    ///Documentation not found in the MISMO model viewer and not provided by Freddie Mac. The value can only be "ReportID"
    #[serde(rename = "ReportIdentifierType")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub report_identifier_type: Option<String>,
    ///Documentation not found in the MISMO model viewer and not provided by Freddie Mac.
    #[serde(rename = "ReportingInformationIdentifier")]
    pub reporting_information_identifier: String,
    ///Documentation not found in the MISMO model viewer and not provided by Freddie Mac.
    #[serde(rename = "ReportingInformationParentIdentifier")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reporting_information_parent_identifier: Option<String>,
}
impl std::fmt::Display for CreditFreddieMacReportingInformation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}