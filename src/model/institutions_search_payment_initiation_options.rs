
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct InstitutionsSearchPaymentInitiationOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consent_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_id: Option<String>,
}
impl std::fmt::Display for InstitutionsSearchPaymentInitiationOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}