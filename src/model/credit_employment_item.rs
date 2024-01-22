use serde::{Serialize, Deserialize};
use super::CreditEmploymentVerification;
///The object containing employment items.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreditEmploymentItem {
    ///Token to represent the underlying Employment data
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub employment_report_token: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub employments: Vec<CreditEmploymentVerification>,
    ///The `item_id` of the Item associated with this webhook, warning, or error
    pub item_id: String,
}
impl std::fmt::Display for CreditEmploymentItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}