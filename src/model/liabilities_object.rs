
use serde::{Serialize, Deserialize};
use super::{CreditCardLiability, MortgageLiability, StudentLoan};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LiabilitiesObject {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credit: Option<Vec<CreditCardLiability>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mortgage: Option<Vec<MortgageLiability>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub student: Option<Vec<StudentLoan>>,
}
impl std::fmt::Display for LiabilitiesObject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}