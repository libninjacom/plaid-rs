
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TransferMetricsGetResponse {
    pub daily_credit_transfer_volume: String,
    pub daily_debit_transfer_volume: String,
    pub iso_currency_code: String,
    pub monthly_transfer_volume: String,
    pub request_id: String,
}
impl std::fmt::Display for TransferMetricsGetResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}