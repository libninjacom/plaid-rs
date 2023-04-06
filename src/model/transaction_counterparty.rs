
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TransactionCounterparty {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logo_url: Option<String>,
    pub name: String,
    #[serde(rename = "type")]
    pub type_: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub website: Option<String>,
}
impl std::fmt::Display for TransactionCounterparty {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}