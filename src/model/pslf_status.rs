use serde::{Serialize, Deserialize};
///Information about the student's eligibility in the Public Service Loan Forgiveness program. This is only returned if the institution is FedLoan (`ins_116527`).
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PslfStatus {
    ///The estimated date borrower will have completed 120 qualifying monthly payments. Returned in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format (YYYY-MM-DD).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub estimated_eligibility_date: Option<chrono::NaiveDate>,
    ///The number of qualifying payments that have been made.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub payments_made: Option<i64>,
    ///The number of qualifying payments remaining.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub payments_remaining: Option<i64>,
}
impl std::fmt::Display for PslfStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}