
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PartnerEndCustomerOAuthStatusUpdatedWebhook {
    pub end_customer_client_id: String,
    pub environment: String,
    pub institution_id: String,
    pub institution_name: String,
    pub status: String,
    pub webhook_code: String,
    pub webhook_type: String,
}
impl std::fmt::Display for PartnerEndCustomerOAuthStatusUpdatedWebhook {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}