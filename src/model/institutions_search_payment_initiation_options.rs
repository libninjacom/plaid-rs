
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct InstitutionsSearchPaymentInitiationOptions {
    pub consent_id: Option<String>,
    pub payment_id: Option<String>,
}
impl std::fmt::Display for InstitutionsSearchPaymentInitiationOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}