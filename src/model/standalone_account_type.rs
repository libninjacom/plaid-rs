
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct StandaloneAccountType {
    pub credit: String,
    pub depository: String,
    pub investment: String,
    pub loan: String,
    pub other: String,
}
impl std::fmt::Display for StandaloneAccountType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}