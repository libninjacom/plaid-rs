
use serde::{Serialize, Deserialize};
use super::CraBankIncome;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CraBankIncomeGetResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_income: Option<Vec<CraBankIncome>>,
    pub request_id: String,
}
impl std::fmt::Display for CraBankIncomeGetResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}