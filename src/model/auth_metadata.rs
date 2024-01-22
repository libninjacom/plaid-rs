use serde::{Serialize, Deserialize};
use super::AuthSupportedMethods;
///Metadata that captures information about the Auth features of an institution.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AuthMetadata {
    ///Metadata specifically related to which auth methods an institution supports.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub supported_methods: Option<AuthSupportedMethods>,
}
impl std::fmt::Display for AuthMetadata {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}