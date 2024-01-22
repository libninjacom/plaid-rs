use serde::{Serialize, Deserialize};
///An optional object for `/credit/payroll_income/refresh` request options.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreditPayrollIncomeRefreshRequestOptions {
    ///An array of `item_id`s to be refreshed. Each `item_id` should uniquely identify a payroll income item. If this field is not provided, all `item_id`s associated with the `user_token` will be refreshed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub item_ids: Option<Vec<String>>,
    ///The URL where Plaid will send the payroll income refresh webhook.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub webhook: Option<String>,
}
impl std::fmt::Display for CreditPayrollIncomeRefreshRequestOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}