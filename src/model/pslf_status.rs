
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PslfStatus {
    pub estimated_eligibility_date: Option<String>,
    pub payments_made: Option<f64>,
    pub payments_remaining: Option<f64>,
}
impl std::fmt::Display for PslfStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}