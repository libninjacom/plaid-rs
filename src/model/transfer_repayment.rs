use serde::{Serialize, Deserialize};
/**A repayment is created automatically after one or more guaranteed transactions receive a return. If there are multiple eligible returns in a day, they are batched together into a single repayment.

Repayments are sent over ACH, with funds typically available on the next banking day.*/
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TransferRepayment {
    ///Decimal amount of the repayment as it appears on your account ledger.
    pub amount: String,
    ///The datetime when the repayment occurred, in RFC 3339 format.
    pub created: chrono::DateTime<chrono::Utc>,
    ///The currency of the repayment, e.g. "USD".
    pub iso_currency_code: String,
    ///Identifier of the repayment.
    pub repayment_id: String,
}
impl std::fmt::Display for TransferRepayment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}