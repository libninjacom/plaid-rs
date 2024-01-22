use serde::{Serialize, Deserialize};
///The number of credits or debits out of the account. This field will only added for depository accounts
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BaseReportNumberFlowInsights {
    ///The number of credits or debits out of the account for this time period.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    /**The end date of this time period.
The date will be returned in an ISO 8601 format (YYYY-MM-DD).*/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end_date: Option<chrono::NaiveDate>,
    /**The start date of this time period.
The date will be returned in an ISO 8601 format (YYYY-MM-DD).*/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_date: Option<chrono::NaiveDate>,
}
impl std::fmt::Display for BaseReportNumberFlowInsights {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}