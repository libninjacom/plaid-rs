
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreditEmploymentVerification {
    pub account_id: Option<String>,
    pub employee_type: Option<String>,
    pub employer: CreditEmployerVerification,
    pub end_date: Option<String>,
    pub last_paystub_date: Option<String>,
    pub platform_ids: CreditPlatformIds,
    pub start_date: Option<String>,
    pub status: Option<String>,
    pub title: Option<String>,
}
impl std::fmt::Display for CreditEmploymentVerification {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}