
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LinkTokenCreateRequestUserStatedIncomeSource {
    pub category: Option<String>,
    pub employer: Option<String>,
    pub pay_annual: Option<f64>,
    pub pay_frequency: Option<String>,
    pub pay_per_cycle: Option<f64>,
    pub pay_type: Option<String>,
}
impl std::fmt::Display for LinkTokenCreateRequestUserStatedIncomeSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}