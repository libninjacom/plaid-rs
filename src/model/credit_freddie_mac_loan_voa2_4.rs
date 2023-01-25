
use serde::{Serialize, Deserialize};
use super::CreditFreddieMacLoanIdentifiersVoa24;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreditFreddieMacLoanVoa24 {
    #[serde(rename = "LOAN_IDENTIFIERS")]
    pub loan_identifiers: CreditFreddieMacLoanIdentifiersVoa24,
    #[serde(rename = "LoanRoleType")]
    pub loan_role_type: String,
}
impl std::fmt::Display for CreditFreddieMacLoanVoa24 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}