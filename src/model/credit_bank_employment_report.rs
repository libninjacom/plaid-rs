use serde::{Serialize, Deserialize};
use super::{CreditBankEmploymentItem, CreditBankEmploymentWarning};
///The report of the Bank Employment data for an end user.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreditBankEmploymentReport {
    ///The unique identifier associated with the Bank Employment Report.
    pub bank_employment_report_id: String,
    ///The number of days requested by the customer for the Bank Employment Report.
    pub days_requested: i64,
    ///The time when the Bank Employment Report was generated, in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format (e.g. "2018-04-12T03:32:11Z").
    pub generated_time: chrono::DateTime<chrono::Utc>,
    ///The list of Items in the report along with the associated metadata about the Item.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<CreditBankEmploymentItem>,
    ///If data from the Bank Employment report was unable to be retrieved, the warnings will contain information about the error that caused the data to be incomplete.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub warnings: Vec<CreditBankEmploymentWarning>,
}
impl std::fmt::Display for CreditBankEmploymentReport {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}