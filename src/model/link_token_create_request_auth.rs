
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LinkTokenCreateRequestAuth {
    pub auth_type_select_enabled: Option<bool>,
    pub automated_microdeposits_enabled: Option<bool>,
    pub flow_type: Option<String>,
    pub instant_match_enabled: Option<bool>,
    pub same_day_microdeposits_enabled: Option<bool>,
}
impl std::fmt::Display for LinkTokenCreateRequestAuth {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}