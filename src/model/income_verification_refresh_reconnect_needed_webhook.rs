use serde::{Serialize, Deserialize};
///Fired when the attempt to refresh Payroll Income data for a user via `/credit/payroll_income/refresh` failed because the user must re-connect their payroll account.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct IncomeVerificationRefreshReconnectNeededWebhook {
    ///The Plaid environment the webhook was sent from
    pub environment: String,
    ///The `user_id` corresponding to the user the webhook has fired for.
    pub user_id: String,
    ///`INCOME_VERIFICATION_REFRESH_RECONNECT_NEEDED`
    pub webhook_code: String,
    ///`INCOME`
    pub webhook_type: String,
}
impl std::fmt::Display for IncomeVerificationRefreshReconnectNeededWebhook {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}