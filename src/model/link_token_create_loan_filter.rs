
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LinkTokenCreateLoanFilter {
    pub account_subtypes: Option<Vec<String>>,
}
impl std::fmt::Display for LinkTokenCreateLoanFilter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}