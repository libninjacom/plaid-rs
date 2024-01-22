use serde::{Serialize, Deserialize};
///Information about an report identifier and a report name.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ReportingInformation {
    ///Documentation not found in the MISMO model viewer and not provided by Freddie Mac.
    #[serde(rename = "ReportingInformationIdentifier")]
    pub reporting_information_identifier: String,
}
impl std::fmt::Display for ReportingInformation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}