use serde::{Serialize, Deserialize};
use super::LoanIdentifier;
///Collection of current and previous identifiers for this loan.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreditFreddieMacLoanIdentifiers {
    #[serde(rename = "LOAN_IDENTIFIER")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub loan_identifier: Vec<LoanIdentifier>,
}
impl std::fmt::Display for CreditFreddieMacLoanIdentifiers {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}