
use serde::{Serialize, Deserialize};
use super::CreditBankEmploymentReport;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreditBankEmploymentGetResponse {
    pub bank_employment_reports: Vec<CreditBankEmploymentReport>,
    pub request_id: String,
}
impl std::fmt::Display for CreditBankEmploymentGetResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}