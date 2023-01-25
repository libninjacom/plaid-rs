
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreditFreddieMacLoanIdentifiersVoa24 {
    #[serde(rename = "LOAN_IDENTIFIER")]
    pub loan_identifier: Vec<LoanIdentifier>,
}
impl std::fmt::Display for CreditFreddieMacLoanIdentifiersVoa24 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}