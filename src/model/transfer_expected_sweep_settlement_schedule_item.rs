use serde::{Serialize, Deserialize};
///Defines an expected sweep date and amount.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TransferExpectedSweepSettlementScheduleItem {
    ///The settlement date of a sweep for this transfer.
    pub sweep_settlement_date: chrono::NaiveDate,
    ///The accumulated amount that has been swept by `sweep_settlement_date`.
    pub swept_settled_amount: String,
}
impl std::fmt::Display for TransferExpectedSweepSettlementScheduleItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}