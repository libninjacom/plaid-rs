
use serde::{Serialize, Deserialize};
use super::{LinkSessionExitMetadata, PlaidError};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LinkSessionExit {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<PlaidError>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<LinkSessionExitMetadata>,
}
impl std::fmt::Display for LinkSessionExit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}