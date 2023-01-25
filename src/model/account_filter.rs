
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AccountFilter {
    pub credit: Option<Vec<String>>,
    pub depository: Option<Vec<String>>,
    pub investment: Option<Vec<String>>,
    pub loan: Option<Vec<String>>,
}
impl std::fmt::Display for AccountFilter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}