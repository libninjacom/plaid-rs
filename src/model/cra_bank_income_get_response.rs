use serde::{Serialize, Deserialize};
use super::CraBankIncome;
///CraBankIncomeGetResponse defines the response schema for `/cra/bank_income/get`.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CraBankIncomeGetResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bank_income: Option<Vec<CraBankIncome>>,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
}
impl std::fmt::Display for CraBankIncomeGetResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}