
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct IdentityVerificationTemplateReference {
    pub id: String,
    pub version: f64,
}
impl std::fmt::Display for IdentityVerificationTemplateReference {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}