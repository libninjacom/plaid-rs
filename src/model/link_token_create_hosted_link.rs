
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LinkTokenCreateHostedLink {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completion_redirect_uri: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_method: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url_lifetime_seconds: Option<i64>,
}
impl std::fmt::Display for LinkTokenCreateHostedLink {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}