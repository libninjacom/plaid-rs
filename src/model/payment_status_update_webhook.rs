
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PaymentStatusUpdateWebhook {
    pub adjusted_reference: Option<String>,
    pub adjusted_start_date: Option<String>,
    pub environment: String,
    pub error: Option<PlaidError>,
    pub new_payment_status: String,
    pub old_payment_status: String,
    pub original_reference: Option<String>,
    pub original_start_date: Option<String>,
    pub payment_id: String,
    pub timestamp: String,
    pub webhook_code: String,
    pub webhook_type: String,
}
impl std::fmt::Display for PaymentStatusUpdateWebhook {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}