
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TransferCreditUsageConfiguration {
    pub expected_average_amount: String,
    pub expected_frequency: String,
    pub expected_highest_amount: String,
    pub expected_monthly_amount: String,
    pub sec_codes: Vec<String>,
}
impl std::fmt::Display for TransferCreditUsageConfiguration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}