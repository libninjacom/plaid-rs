use serde::{Serialize, Deserialize};
///Fired when a change to the user's income is detected. You should call `/credit/bank_income/refresh` to get updated income data for the user. To receive this webhook, subscribe in the [Dashboard](https://dashboard.plaid.com/developers/webhooks).
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BankIncomeRefreshUpdateWebhook {
    ///The Plaid environment the webhook was sent from
    pub environment: String,
    ///The `user_id` corresponding to the user the webhook has fired for.
    pub user_id: String,
    ///`BANK_INCOME_REFRESH_UPDATE`
    pub webhook_code: String,
    ///`INCOME`
    pub webhook_type: String,
}
impl std::fmt::Display for BankIncomeRefreshUpdateWebhook {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}