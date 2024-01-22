use serde::{Serialize, Deserialize};
///Describes the last time each datatype was accessed by an application.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LastDataAccessTimes {
    ///The last time account_balance_info was accessed by this application in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format in UTC. null if never accessed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub account_balance_info: Option<chrono::DateTime<chrono::Utc>>,
    ///The last time account_routing_number was accessed by this application in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format in UTC. null if never accessed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub account_routing_number: Option<chrono::DateTime<chrono::Utc>>,
    ///ID of the application accessing data.
    pub application_id: String,
    ///The last time contact_details was accessed by this application in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format in UTC. null if never accessed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contact_details: Option<chrono::DateTime<chrono::Utc>>,
    ///The last time credit_and_loans was accessed by this application in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format in UTC. null if never accessed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub credit_and_loans: Option<chrono::DateTime<chrono::Utc>>,
    ///The last time investments was accessed by this application in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format in UTC. null if never accessed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub investments: Option<chrono::DateTime<chrono::Utc>>,
    ///The last time payroll_info was accessed by this application in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format in UTC. null if never accessed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub payroll_info: Option<chrono::DateTime<chrono::Utc>>,
    ///The last time transaction_risk_info was accessed by this application in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format in UTC. null if never accessed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transaction_risk_info: Option<chrono::DateTime<chrono::Utc>>,
    ///The last time transactions was accessed by this application in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format in UTC. null if never accessed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transactions: Option<chrono::DateTime<chrono::Utc>>,
}
impl std::fmt::Display for LastDataAccessTimes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}