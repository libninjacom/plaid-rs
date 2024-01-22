use serde::{Serialize, Deserialize};
use super::CreditFreddieMacLoan;
///A collection of loans that are part of a single deal.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreditFreddieMacLoans {
    ///Information specific to a mortgage loan agreement between one or more borrowers and a mortgage lender.
    #[serde(rename = "LOAN")]
    pub loan: CreditFreddieMacLoan,
}
impl std::fmt::Display for CreditFreddieMacLoans {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}