
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct IncomeVerificationCreateRequest {
    pub options: Option<IncomeVerificationCreateRequestOptions>,
    pub precheck_id: Option<String>,
    pub webhook: String,
}
impl std::fmt::Display for IncomeVerificationCreateRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}