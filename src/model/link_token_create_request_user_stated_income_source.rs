
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LinkTokenCreateRequestUserStatedIncomeSource {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub employer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pay_annual: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pay_frequency: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pay_per_cycle: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pay_type: Option<String>,
}
impl std::fmt::Display for LinkTokenCreateRequestUserStatedIncomeSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}