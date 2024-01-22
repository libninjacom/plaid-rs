use serde::{Serialize, Deserialize};
use super::CreditFreddieMacLoanIdentifiers;
///Information specific to a mortgage loan agreement between one or more borrowers and a mortgage lender.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreditFreddieMacLoan {
    ///Collection of current and previous identifiers for this loan.
    #[serde(rename = "LOAN_IDENTIFIERS")]
    pub loan_identifiers: CreditFreddieMacLoanIdentifiers,
    ///Type of loan. The value can only be "SubjectLoan"
    #[serde(rename = "LoanRoleType")]
    pub loan_role_type: String,
}
impl std::fmt::Display for CreditFreddieMacLoan {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}