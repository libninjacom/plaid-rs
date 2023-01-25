
use serde::{Serialize, Deserialize};
use super::CreditEmploymentItem;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreditEmploymentGetResponse {
    pub items: Vec<CreditEmploymentItem>,
    pub request_id: String,
}
impl std::fmt::Display for CreditEmploymentGetResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}