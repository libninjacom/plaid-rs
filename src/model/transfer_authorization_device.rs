
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TransferAuthorizationDevice {
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
}
impl std::fmt::Display for TransferAuthorizationDevice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}