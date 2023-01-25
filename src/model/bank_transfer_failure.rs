
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BankTransferFailure {
    pub ach_return_code: Option<String>,
    pub description: Option<String>,
}
impl std::fmt::Display for BankTransferFailure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}