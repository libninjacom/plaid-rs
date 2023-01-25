
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WatchlistScreeningDocument {
    pub number: String,
    #[serde(rename = "type")]
    pub type_: String,
}
impl std::fmt::Display for WatchlistScreeningDocument {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}