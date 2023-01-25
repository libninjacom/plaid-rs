
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct InstitutionsSearchAccountFilter {
    pub credit: Option<Vec<AccountSubtype>>,
    pub depository: Option<Vec<AccountSubtype>>,
    pub investment: Option<Vec<AccountSubtype>>,
    pub loan: Option<Vec<AccountSubtype>>,
}
impl std::fmt::Display for InstitutionsSearchAccountFilter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}