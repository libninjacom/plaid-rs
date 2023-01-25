
use serde::{Serialize, Deserialize};
use super::{CreditCardLiability, MortgageLiability, StudentLoan};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LiabilitiesObject {
    pub credit: Option<Vec<CreditCardLiability>>,
    pub mortgage: Option<Vec<MortgageLiability>>,
    pub student: Option<Vec<StudentLoan>>,
}
impl std::fmt::Display for LiabilitiesObject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}