
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SignalEvaluateRequest {
    pub access_token: String,
    pub account_id: String,
    pub amount: f64,
    pub client_transaction_id: String,
    pub client_user_id: Option<String>,
    pub default_payment_method: Option<String>,
    pub device: Option<SignalDevice>,
    pub is_recurring: Option<bool>,
    pub user: Option<SignalUser>,
    pub user_present: Option<bool>,
}
impl std::fmt::Display for SignalEvaluateRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}