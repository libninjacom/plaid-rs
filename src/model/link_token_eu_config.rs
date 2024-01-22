use serde::{Serialize, Deserialize};
///Configuration parameters for EU flows
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LinkTokenEuConfig {
    ///If `true`, open Link without an initial UI. Defaults to `false`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub headless: Option<bool>,
}
impl std::fmt::Display for LinkTokenEuConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}