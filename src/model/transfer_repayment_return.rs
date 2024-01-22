use serde::{Serialize, Deserialize};
///Represents a return on a Guaranteed ACH transfer that is included in the specified repayment.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TransferRepaymentReturn {
    ///The value of the returned transfer.
    pub amount: String,
    ///The unique identifier of the corresponding `returned` transfer event.
    pub event_id: i64,
    ///The currency of the repayment, e.g. "USD".
    pub iso_currency_code: String,
    ///The unique identifier of the guaranteed transfer that was returned.
    pub transfer_id: String,
}
impl std::fmt::Display for TransferRepaymentReturn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}