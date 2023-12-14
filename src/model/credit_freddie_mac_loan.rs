
use serde::{Serialize, Deserialize};
use super::CreditFreddieMacLoanIdentifiers;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreditFreddieMacLoan {
    #[serde(rename = "LOAN_IDENTIFIERS")]
    pub loan_identifiers: CreditFreddieMacLoanIdentifiers,
    #[serde(rename = "LoanRoleType")]
    pub loan_role_type: String,
}
impl std::fmt::Display for CreditFreddieMacLoan {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}