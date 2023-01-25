
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MortgageInterestRate {
    pub percentage: Option<f64>,
    #[serde(rename = "type")]
    pub type_: Option<String>,
}
impl std::fmt::Display for MortgageInterestRate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}