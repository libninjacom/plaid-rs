
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct InvestmentsTransactionsGetRequest {
    pub access_token: String,
    pub end_date: String,
    pub options: Option<InvestmentsTransactionsGetRequestOptions>,
    pub start_date: String,
}
impl std::fmt::Display for InvestmentsTransactionsGetRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}