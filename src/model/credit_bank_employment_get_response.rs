use serde::{Serialize, Deserialize};
use super::CreditBankEmploymentReport;
///CreditBankEmploymentGetResponse defines the response schema for `/beta/credit/v1/bank_employment/get`.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreditBankEmploymentGetResponse {
    ///Bank Employment data. Each entry in the array will be a distinct bank employment report.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub bank_employment_reports: Vec<CreditBankEmploymentReport>,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
}
impl std::fmt::Display for CreditBankEmploymentGetResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}