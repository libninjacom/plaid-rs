use serde::{Serialize, Deserialize};
///A representation of a removed transaction
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RemovedTransaction {
    ///The ID of the removed transaction.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transaction_id: Option<String>,
}
impl std::fmt::Display for RemovedTransaction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}