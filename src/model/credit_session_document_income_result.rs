
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreditSessionDocumentIncomeResult {
    pub num_paystubs_uploaded: i64,
    #[serde(rename = "num_w2s_uploaded")]
    pub num_w2_s_uploaded: i64,
}
impl std::fmt::Display for CreditSessionDocumentIncomeResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}