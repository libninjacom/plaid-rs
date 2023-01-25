
use serde::{Serialize, Deserialize};
use super::PaystubOverride;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct IncomeOverride {
    pub paystubs: Option<Vec<PaystubOverride>>,
}
impl std::fmt::Display for IncomeOverride {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}