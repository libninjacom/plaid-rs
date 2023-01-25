
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SandboxPublicTokenCreateRequestOptionsTransactions {
    pub end_date: Option<String>,
    pub start_date: Option<String>,
}
impl std::fmt::Display for SandboxPublicTokenCreateRequestOptionsTransactions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}