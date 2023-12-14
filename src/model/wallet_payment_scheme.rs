
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WalletPaymentScheme(pub serde_json::Value);