use serde::{Serialize, Deserialize};
use super::LoanIdentifier;
///Collection of current and previous identifiers for this loan.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LoanIdentifiers {
    ///The information used to identify this loan by various parties to the transaction or other organizations.
    #[serde(rename = "LOAN_IDENTIFIER")]
    pub loan_identifier: LoanIdentifier,
}
impl std::fmt::Display for LoanIdentifiers {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}