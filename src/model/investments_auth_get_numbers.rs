
use serde::{Serialize, Deserialize};
use super::{NumbersAcats, NumbersAton};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct InvestmentsAuthGetNumbers {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acats: Option<Vec<NumbersAcats>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aton: Option<Vec<NumbersAton>>,
}
impl std::fmt::Display for InvestmentsAuthGetNumbers {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}