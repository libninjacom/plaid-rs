
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LinkTokenCreateRequestPaymentInitiation {
    pub consent_id: Option<String>,
    pub payment_id: Option<String>,
}
impl std::fmt::Display for LinkTokenCreateRequestPaymentInitiation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}