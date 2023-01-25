
use serde::{Serialize, Deserialize};
use super::LoanIdentifier;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoanIdentifiers {
    #[serde(rename = "LOAN_IDENTIFIER")]
    pub loan_identifier: LoanIdentifier,
}
impl std::fmt::Display for LoanIdentifiers {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}