
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PaystubDeduction {
    pub is_pretax: Option<bool>,
    pub total: Option<f64>,
    #[serde(rename = "type")]
    pub type_: Option<String>,
}
impl std::fmt::Display for PaystubDeduction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}