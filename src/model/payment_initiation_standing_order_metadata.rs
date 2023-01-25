
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PaymentInitiationStandingOrderMetadata {
    pub supports_standing_order_end_date: bool,
    pub supports_standing_order_negative_execution_days: bool,
    pub valid_standing_order_intervals: Vec<String>,
}
impl std::fmt::Display for PaymentInitiationStandingOrderMetadata {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}