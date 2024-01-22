use serde::{Serialize, Deserialize};
use super::TransferRefundFailure;
///Represents a refund within the Transfers API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TransferRefund {
    ///The amount of the refund (decimal string with two digits of precision e.g. "10.00").
    pub amount: String,
    ///The datetime when this refund was created. This will be of the form `2006-01-02T15:04:05Z`
    pub created: chrono::DateTime<chrono::Utc>,
    ///The failure reason if the event type for a refund is `"failed"` or `"returned"`. Null value otherwise.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<TransferRefundFailure>,
    ///Plaid’s unique identifier for a refund.
    pub id: String,
    /**The trace identifier for the transfer based on its network. This will only be set after the transfer has posted.

For `ach` or `same-day-ach` transfers, this is the ACH trace number. Currently, the field will remain null for transfers on other rails.*/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub network_trace_id: Option<String>,
    /**The status of the refund.

`pending`: A new refund was created; it is in the pending state.
`posted`: The refund has been successfully submitted to the payment network.
`settled`: Credits have been refunded to the Plaid linked account.
`cancelled`: The refund was cancelled by the client.
`failed`: The refund has failed.
`returned`: The refund was returned.*/
    pub status: String,
    ///The ID of the transfer to refund.
    pub transfer_id: String,
}
impl std::fmt::Display for TransferRefund {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}