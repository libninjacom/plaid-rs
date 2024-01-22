use serde::{Serialize, Deserialize};
///BankTransferSweep describes a sweep transfer.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BankTransferSweep {
    ///The amount of the sweep.
    pub amount: String,
    ///The datetime when the sweep occurred, in RFC 3339 format.
    pub created_at: chrono::DateTime<chrono::Utc>,
    ///Identifier of the sweep.
    pub id: String,
    ///The currency of the sweep, e.g. "USD".
    pub iso_currency_code: String,
}
impl std::fmt::Display for BankTransferSweep {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}