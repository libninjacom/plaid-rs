
use serde::{Serialize, Deserialize};
use super::{CreditEmployerVerification, CreditPlatformIds};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreditEmploymentVerification {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub employee_type: Option<String>,
    pub employer: CreditEmployerVerification,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<chrono::NaiveDate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_paystub_date: Option<chrono::NaiveDate>,
    pub platform_ids: CreditPlatformIds,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<chrono::NaiveDate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}
impl std::fmt::Display for CreditEmploymentVerification {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}