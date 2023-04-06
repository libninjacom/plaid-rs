
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LinkTokenEuConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headless: Option<bool>,
}
impl std::fmt::Display for LinkTokenEuConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}