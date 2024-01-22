use serde::{Serialize, Deserialize};
use super::SweepStatus;
/**Describes a sweep of funds to / from the sweep account.

A sweep is associated with many sweep events (events of type `swept` or `return_swept`) which can be retrieved by invoking the `/transfer/event/list` endpoint with the corresponding `sweep_id`.

`swept` events occur when the transfer amount is credited or debited from your sweep account, depending on the `type` of the transfer. `return_swept` events occur when a transfer is returned and Plaid undoes the credit or debit.

The total sum of the `swept` and `return_swept` events is equal to the `amount` of the sweep Plaid creates and matches the amount of the entry on your sweep account ledger.*/
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TransferSweep {
    /**Signed decimal amount of the sweep as it appears on your sweep account ledger (e.g. "-10.00")

If amount is not present, the sweep was net-settled to zero and outstanding debits and credits between the sweep account and Plaid are balanced.*/
    pub amount: String,
    ///The datetime when the sweep occurred, in RFC 3339 format.
    pub created: chrono::DateTime<chrono::Utc>,
    ///The description of the deposit that will be passed to the receiving bank (up to 10 characters). Note that banks utilize this field differently, and may or may not show it on the bank statement.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    ///The id of the funding account to use, available in the Plaid Dashboard. This determines which of your business checking accounts will be credited or debited.
    pub funding_account_id: String,
    ///Identifier of the sweep.
    pub id: String,
    ///The currency of the sweep, e.g. "USD".
    pub iso_currency_code: String,
    /**The trace identifier for the transfer based on its network. This will only be set after the transfer has posted.

For `ach` or `same-day-ach` transfers, this is the ACH trace number. Currently, the field will remain null for transfers on other rails.*/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub network_trace_id: Option<String>,
    ///The date when the sweep settled, in the YYYY-MM-DD format.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub settled: Option<chrono::NaiveDate>,
    /**The status of a sweep transfer

`"pending"` - The sweep is currently pending
`"posted"` - The sweep has been posted
`"settled"` - The sweep has settled
`"returned"` - The sweep has been returned
`"failed"` - The sweep has failed*/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<SweepStatus>,
    /**The trigger of the sweep

`"manual"` - The sweep is created manually by the customer
`"incoming"` - The sweep is created by incoming funds flow (e.g. Incoming Wire)
`"balance_threshold"` - The sweep is created by balance threshold setting
`"automatic_aggregate"` - The sweep is created by the Plaid automatic aggregation process. These funds did not pass through the Plaid Ledger balance.*/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub trigger: Option<String>,
}
impl std::fmt::Display for TransferSweep {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}