
use serde::{Serialize, Deserialize};
use super::PaystubPayFrequency;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PaystubDetails {
    pub pay_date: Option<String>,
    pub pay_frequency: Option<PaystubPayFrequency>,
    pub pay_period_end_date: Option<String>,
    pub pay_period_start_date: Option<String>,
    pub paystub_provider: Option<String>,
}
impl std::fmt::Display for PaystubDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}