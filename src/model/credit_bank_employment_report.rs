
use serde::{Serialize, Deserialize};
use super::{CreditBankEmploymentItem, CreditBankEmploymentWarning};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreditBankEmploymentReport {
    pub bank_employment_report_id: String,
    pub days_requested: i64,
    pub generated_time: chrono::DateTime<chrono::Utc>,
    pub items: Vec<CreditBankEmploymentItem>,
    pub warnings: Vec<CreditBankEmploymentWarning>,
}
impl std::fmt::Display for CreditBankEmploymentReport {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}