use serde::{Serialize, Deserialize};
use super::LoanIdentifiers;
///Information specific to a mortgage loan agreement between one or more borrowers and a mortgage lender.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Loan {
    ///Collection of current and previous identifiers for this loan.
    #[serde(rename = "LOAN_IDENTIFIERS")]
    pub loan_identifiers: LoanIdentifiers,
}
impl std::fmt::Display for Loan {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}