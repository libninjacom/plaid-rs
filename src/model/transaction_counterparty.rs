
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TransactionCounterparty {
    pub logo_url: Option<String>,
    pub name: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub website: Option<String>,
}
impl std::fmt::Display for TransactionCounterparty {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}