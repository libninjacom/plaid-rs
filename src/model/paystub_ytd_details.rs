
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PaystubYtdDetails {
    pub gross_earnings: Option<f64>,
    pub net_earnings: Option<f64>,
}
impl std::fmt::Display for PaystubYtdDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}