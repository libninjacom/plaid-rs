
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PslfStatus {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_eligibility_date: Option<chrono::NaiveDate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payments_made: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payments_remaining: Option<f64>,
}
impl std::fmt::Display for PslfStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}