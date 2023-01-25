
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LinkTokenCreateInvestmentFilter {
    pub account_subtypes: Option<Vec<String>>,
}
impl std::fmt::Display for LinkTokenCreateInvestmentFilter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}