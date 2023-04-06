
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Apr {
    pub apr_percentage: f64,
    pub apr_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub balance_subject_to_apr: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interest_charge_amount: Option<f64>,
}
impl std::fmt::Display for Apr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}