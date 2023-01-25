
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SignalEvaluateRequired {
    pub access_token: String,
    pub account_id: String,
    pub amount: f64,
    pub client_transaction_id: String,
}
impl std::fmt::Display for SignalEvaluateRequired {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}