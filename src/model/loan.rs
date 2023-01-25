
use serde::{Serialize, Deserialize};
use super::LoanIdentifiers;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Loan {
    #[serde(rename = "LOAN_IDENTIFIERS")]
    pub loan_identifiers: LoanIdentifiers,
}
impl std::fmt::Display for Loan {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}