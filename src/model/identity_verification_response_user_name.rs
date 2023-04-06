
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct IdentityVerificationResponseUserName {
    pub family_name: String,
    pub given_name: String,
}
impl std::fmt::Display for IdentityVerificationResponseUserName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}