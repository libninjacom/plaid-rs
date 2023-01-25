
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreditBankIncomeAccount {
    pub account_id: Option<String>,
    pub mask: Option<String>,
    pub name: Option<String>,
    pub official_name: Option<String>,
    pub owners: Option<Vec<Owner>>,
    pub subtype: Option<String>,
    #[serde(rename = "type")]
    pub type_: Option<String>,
}
impl std::fmt::Display for CreditBankIncomeAccount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}