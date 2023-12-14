
use serde::{Serialize, Deserialize};
use super::LoanIdentifier;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreditFreddieMacLoanIdentifiers {
    #[serde(rename = "LOAN_IDENTIFIER")]
    pub loan_identifier: Vec<LoanIdentifier>,
}
impl std::fmt::Display for CreditFreddieMacLoanIdentifiers {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}