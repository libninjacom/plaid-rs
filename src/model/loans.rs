use serde::{Serialize, Deserialize};
use super::Loan;
///A collection of loans that are part of a single deal.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Loans {
    ///Information specific to a mortgage loan agreement between one or more borrowers and a mortgage lender.
    #[serde(rename = "LOAN")]
    pub loan: Loan,
}
impl std::fmt::Display for Loans {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}