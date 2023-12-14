
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LinkTokenCreateRequestIncomeVerificationPayrollIncome {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_types: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_update_mode: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_id_to_update: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parsing_config: Option<Vec<String>>,
}
impl std::fmt::Display for LinkTokenCreateRequestIncomeVerificationPayrollIncome {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}