
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct IncomeVerificationCreateRequestOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_tokens: Option<Vec<String>>,
}
impl std::fmt::Display for IncomeVerificationCreateRequestOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}