
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PaymentConsentValidDateTime {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<chrono::DateTime<chrono::Utc>>,
}
impl std::fmt::Display for PaymentConsentValidDateTime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}