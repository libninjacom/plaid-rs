use serde::{Serialize, Deserialize};
use super::ProductStatusBreakdown;
///A representation of the status health of a request type. Auth requests, Balance requests, Identity requests, Investments requests, Liabilities requests, Transactions updates, Investments updates, Liabilities updates, and Item logins each have their own status object.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ProductStatus {
    ///A detailed breakdown of the institution's performance for a request type. The values for `success`, `error_plaid`, and `error_institution` sum to 1. The time range used for calculating the breakdown may range from the most recent few minutes to the past six hours. In general, smaller institutions will show status that was calculated over a longer period of time. For Investment updates, which are refreshed less frequently, the period assessed may be 24 hours or more. For more details, see [Institution status details](https://plaid.com/docs/account/activity/#institution-status-details).
    pub breakdown: ProductStatusBreakdown,
    ///[ISO 8601](https://wikipedia.org/wiki/ISO_8601) formatted timestamp of the last status change for the institution.
    pub last_status_change: chrono::DateTime<chrono::Utc>,
    /**This field is deprecated in favor of the `breakdown` object, which provides more granular institution health data.

`HEALTHY`: the majority of requests are successful
`DEGRADED`: only some requests are successful
`DOWN`: all requests are failing*/
    pub status: String,
}
impl std::fmt::Display for ProductStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}