
use serde::{Serialize, Deserialize};
use super::Status;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Statuses {
    #[serde(rename = "STATUS")]
    pub status: Status,
}
impl std::fmt::Display for Statuses {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}