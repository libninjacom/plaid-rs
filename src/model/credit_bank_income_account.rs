
use serde::{Serialize, Deserialize};
use super::Owner;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreditBankIncomeAccount {
    pub account_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mask: Option<String>,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub official_name: Option<String>,
    pub owners: Vec<Owner>,
    pub subtype: String,
    #[serde(rename = "type")]
    pub type_: String,
}
impl std::fmt::Display for CreditBankIncomeAccount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}