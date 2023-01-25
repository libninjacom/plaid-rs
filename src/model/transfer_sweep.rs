
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TransferSweep {
    pub amount: String,
    pub created: String,
    pub id: String,
    pub iso_currency_code: String,
    pub settled: Option<String>,
}
impl std::fmt::Display for TransferSweep {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}