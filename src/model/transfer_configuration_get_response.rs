
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TransferConfigurationGetResponse {
    pub iso_currency_code: String,
    pub max_daily_credit_amount: String,
    pub max_daily_debit_amount: String,
    pub max_monthly_amount: String,
    pub max_monthly_credit_amount: String,
    pub max_monthly_debit_amount: String,
    pub max_single_transfer_amount: String,
    pub max_single_transfer_credit_amount: String,
    pub max_single_transfer_debit_amount: String,
    pub request_id: String,
}
impl std::fmt::Display for TransferConfigurationGetResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}