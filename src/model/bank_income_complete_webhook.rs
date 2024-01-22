use serde::{Serialize, Deserialize};
///Fired when a bank income report has finished generating or failed to generate, triggered by calling `/credit/bank_income/get` in CRA enabled client.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BankIncomeCompleteWebhook {
    ///The Plaid environment the webhook was sent from
    pub environment: String,
    /**The result of the bank income report generation

`SUCCESS`: The bank income report was successfully generated and can be retrieved via `/credit/bank_income/get`.

`FAILURE`: The bank income report failed to be generated*/
    pub result: String,
    ///The `user_id` corresponding to the user the webhook has fired for.
    pub user_id: String,
    ///`BANK_INCOME_COMPLETE`
    pub webhook_code: String,
    ///`INCOME`
    pub webhook_type: String,
}
impl std::fmt::Display for BankIncomeCompleteWebhook {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}