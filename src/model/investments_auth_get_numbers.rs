use serde::{Serialize, Deserialize};
use super::{NumbersAcats, NumbersAton};
///Identifying information for transferring holdings to an investments account.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct InvestmentsAuthGetNumbers {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub acats: Option<Vec<NumbersAcats>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub aton: Option<Vec<NumbersAton>>,
}
impl std::fmt::Display for InvestmentsAuthGetNumbers {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}