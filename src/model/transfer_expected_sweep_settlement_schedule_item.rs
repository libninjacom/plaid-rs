
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferExpectedSweepSettlementScheduleItem {
    pub sweep_settlement_date: chrono::NaiveDate,
    pub swept_settled_amount: String,
}
impl std::fmt::Display for TransferExpectedSweepSettlementScheduleItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}