
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreditBankIncomeGetRequestOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
}
impl std::fmt::Display for CreditBankIncomeGetRequestOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}