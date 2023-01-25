
use serde::{Serialize, Deserialize};
use super::IncomeBreakdownType;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct IncomeBreakdown {
    pub hours: Option<f64>,
    pub rate: Option<f64>,
    pub total: Option<f64>,
    #[serde(rename = "type")]
    pub type_: Option<IncomeBreakdownType>,
}
impl std::fmt::Display for IncomeBreakdown {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}