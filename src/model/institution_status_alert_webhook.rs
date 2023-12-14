
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct InstitutionStatusAlertWebhook {
    pub environment: String,
    pub institution_id: String,
    pub institution_overall_success_rate: f64,
    pub webhook_code: String,
    pub webhook_type: String,
}
impl std::fmt::Display for InstitutionStatusAlertWebhook {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}