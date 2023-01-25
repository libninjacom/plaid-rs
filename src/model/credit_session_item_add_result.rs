
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreditSessionItemAddResult {
    pub institution_id: Option<String>,
    pub item_id: Option<String>,
    pub public_token: Option<String>,
}
impl std::fmt::Display for CreditSessionItemAddResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}