use serde::{Serialize, Deserialize};
use super::{CreditCardLiability, MortgageLiability, StudentLoan};
///An object containing liability accounts
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LiabilitiesObject {
    ///The credit accounts returned.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub credit: Option<Vec<CreditCardLiability>>,
    ///The mortgage accounts returned.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mortgage: Option<Vec<MortgageLiability>>,
    ///The student loan accounts returned.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub student: Option<Vec<StudentLoan>>,
}
impl std::fmt::Display for LiabilitiesObject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}