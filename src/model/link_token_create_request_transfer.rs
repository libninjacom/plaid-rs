
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LinkTokenCreateRequestTransfer {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intent_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_profile_token: Option<String>,
}
impl std::fmt::Display for LinkTokenCreateRequestTransfer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}