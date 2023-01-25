
use serde::{Serialize, Deserialize};
use super::CreditEmploymentVerification;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreditEmploymentItem {
    pub employment_report_token: Option<String>,
    pub employments: Vec<CreditEmploymentVerification>,
    pub item_id: String,
}
impl std::fmt::Display for CreditEmploymentItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}