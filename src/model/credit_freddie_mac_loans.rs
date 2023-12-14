
use serde::{Serialize, Deserialize};
use super::CreditFreddieMacLoan;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreditFreddieMacLoans {
    #[serde(rename = "LOAN")]
    pub loan: CreditFreddieMacLoan,
}
impl std::fmt::Display for CreditFreddieMacLoans {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}