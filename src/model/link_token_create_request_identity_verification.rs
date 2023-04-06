
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LinkTokenCreateRequestIdentityVerification {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consent: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gave_consent: Option<bool>,
    pub template_id: String,
}
impl std::fmt::Display for LinkTokenCreateRequestIdentityVerification {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}