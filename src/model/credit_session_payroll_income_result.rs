
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreditSessionPayrollIncomeResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub institution_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub institution_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_paystubs_retrieved: Option<i64>,
    #[serde(rename = "num_w2s_retrieved")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_w2_s_retrieved: Option<i64>,
}
impl std::fmt::Display for CreditSessionPayrollIncomeResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}