
use serde::{Serialize, Deserialize};
use super::LinkSessionSuccessMetadata;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LinkSessionSuccess {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<LinkSessionSuccessMetadata>,
    pub public_token: String,
}
impl std::fmt::Display for LinkSessionSuccess {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}