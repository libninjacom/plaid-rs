
use serde::{Serialize, Deserialize};
use super::CreditFreddieMacLoanVoa24;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreditFreddieMacLoansVoa24 {
    #[serde(rename = "LOAN")]
    pub loan: CreditFreddieMacLoanVoa24,
}
impl std::fmt::Display for CreditFreddieMacLoansVoa24 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}