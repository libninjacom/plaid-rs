use serde::{Serialize, Deserialize};
///A detailed breakdown of the institution's performance for a request type. The values for `success`, `error_plaid`, and `error_institution` sum to 1. The time range used for calculating the breakdown may range from the most recent few minutes to the past six hours. In general, smaller institutions will show status that was calculated over a longer period of time. For Investment updates, which are refreshed less frequently, the period assessed may be 24 hours or more. For more details, see [Institution status details](https://plaid.com/docs/account/activity/#institution-status-details).
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ProductStatusBreakdown {
    ///The percentage of logins that are failing due to an issue in the institution's system, expressed as a decimal.
    pub error_institution: f64,
    ///The percentage of logins that are failing due to an internal Plaid issue, expressed as a decimal.
    pub error_plaid: f64,
    ///The `refresh_interval` may be `DELAYED` or `STOPPED` even when the success rate is high. This value is only returned for Transactions status breakdowns.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub refresh_interval: Option<String>,
    ///The percentage of login attempts that are successful, expressed as a decimal.
    pub success: f64,
}
impl std::fmt::Display for ProductStatusBreakdown {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}