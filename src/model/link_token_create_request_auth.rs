
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LinkTokenCreateRequestAuth {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_type_select_enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automated_microdeposits_enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instant_match_enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub same_day_microdeposits_enabled: Option<bool>,
}
impl std::fmt::Display for LinkTokenCreateRequestAuth {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}