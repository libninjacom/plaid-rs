
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EmailAddressMatchScore {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score: Option<i64>,
}
impl std::fmt::Display for EmailAddressMatchScore {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}