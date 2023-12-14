
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PersonalFinanceCategory {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidence_level: Option<String>,
    pub detailed: String,
    pub primary: String,
}
impl std::fmt::Display for PersonalFinanceCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}