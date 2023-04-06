
use serde::{Serialize, Deserialize};
use super::PlaidError;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentStatusUpdateWebhook {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adjusted_reference: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adjusted_start_date: Option<chrono::NaiveDate>,
    pub environment: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<PlaidError>,
    pub new_payment_status: String,
    pub old_payment_status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub original_reference: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub original_start_date: Option<chrono::NaiveDate>,
    pub payment_id: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_id: Option<String>,
    pub webhook_code: String,
    pub webhook_type: String,
}
impl std::fmt::Display for PaymentStatusUpdateWebhook {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}