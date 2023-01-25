
use serde::{Serialize, Deserialize};
use super::{Pay, TotalCanonicalDescription};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Total {
    pub canonical_description: Option<TotalCanonicalDescription>,
    pub current_pay: Option<Pay>,
    pub description: Option<String>,
    pub ytd_pay: Option<Pay>,
}
impl std::fmt::Display for Total {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}