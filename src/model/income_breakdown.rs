
use serde::{Serialize, Deserialize};
use super::IncomeBreakdownType;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct IncomeBreakdown {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hours: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<f64>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<IncomeBreakdownType>,
}
impl std::fmt::Display for IncomeBreakdown {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}