use serde::{Serialize, Deserialize};
/**This webhook is only sent to [Plaid processor partners](https://plaid.com/docs/auth/partnerships/).

Fired when recurring transactions data is updated. This includes when a new recurring stream is detected or when a new transaction is added to an existing recurring stream. The `RECURRING_TRANSACTIONS_UPDATE` webhook will also fire when one or more attributes of the recurring stream changes, which is usually a result of the addition, update, or removal of transactions to the stream.

After receipt of this webhook, the updated data can be fetched from `/processor/transactions/recurring/get`.*/
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ProcessorRecurringTransactionsUpdateWebhook {
    ///The ID of the account.
    pub account_id: String,
    ///The Plaid environment the webhook was sent from
    pub environment: String,
    ///`RECURRING_TRANSACTIONS_UPDATE`
    pub webhook_code: String,
    ///`TRANSACTIONS`
    pub webhook_type: String,
}
impl std::fmt::Display for ProcessorRecurringTransactionsUpdateWebhook {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}