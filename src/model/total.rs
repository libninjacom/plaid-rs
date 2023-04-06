
use serde::{Serialize, Deserialize};
use super::{Pay, TotalCanonicalDescription};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Total {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub canonical_description: Option<TotalCanonicalDescription>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_pay: Option<Pay>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ytd_pay: Option<Pay>,
}
impl std::fmt::Display for Total {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}