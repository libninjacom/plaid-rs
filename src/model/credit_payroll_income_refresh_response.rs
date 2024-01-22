use serde::{Serialize, Deserialize};
///CreditPayrollIncomeRefreshResponse defines the response schema for `/credit/payroll_income/refresh`
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreditPayrollIncomeRefreshResponse {
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
    /**The verification refresh status. One of the following:

`"USER_PRESENCE_REQUIRED"` User presence is required to refresh an income verification.
`"SUCCESSFUL"` The income verification refresh was successful.
`"NOT_FOUND"` No new data was found after the income verification refresh.*/
    pub verification_refresh_status: String,
}
impl std::fmt::Display for CreditPayrollIncomeRefreshResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}