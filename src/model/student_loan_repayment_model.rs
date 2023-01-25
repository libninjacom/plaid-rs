
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct StudentLoanRepaymentModel {
    pub non_repayment_months: f64,
    pub repayment_months: f64,
    #[serde(rename = "type")]
    pub type_: String,
}
impl std::fmt::Display for StudentLoanRepaymentModel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}