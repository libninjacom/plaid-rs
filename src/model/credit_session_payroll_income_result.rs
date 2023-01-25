
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreditSessionPayrollIncomeResult {
    pub institution_id: Option<String>,
    pub institution_name: Option<String>,
    pub num_paystubs_retrieved: Option<i64>,
    #[serde(rename = "num_w2s_retrieved")]
    pub num_w2_s_retrieved: Option<i64>,
}
impl std::fmt::Display for CreditSessionPayrollIncomeResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}