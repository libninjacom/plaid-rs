
use serde::{Serialize, Deserialize};
use super::AuthSupportedMethods;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AuthMetadata {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supported_methods: Option<AuthSupportedMethods>,
}
impl std::fmt::Display for AuthMetadata {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}