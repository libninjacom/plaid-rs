use serde::{Serialize, Deserialize};
///Fired when refreshed statements extraction is completed or failed to be completed. Triggered by calling `/statements/refresh`.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct StatementsRefreshCompleteWebhook {
    ///The Plaid environment the webhook was sent from
    pub environment: String,
    ///The Plaid Item ID. The `item_id` is always unique; linking the same account at the same institution twice will result in two Items with different `item_id` values. Like all Plaid identifiers, the `item_id` is case-sensitive.
    pub item_id: String,
    /**The result of the statement refresh extraction

`SUCCESS`: The statements were successfully extracted and can be listed via `/statements/list/` and downloaded via `/statements/download/`.

`FAILURE`: The statements failed to be extracted.*/
    pub result: String,
    ///`STATEMENTS_REFRESH_COMPLETE`
    pub webhook_code: String,
    ///`STATEMENTS`
    pub webhook_type: String,
}
impl std::fmt::Display for StatementsRefreshCompleteWebhook {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}