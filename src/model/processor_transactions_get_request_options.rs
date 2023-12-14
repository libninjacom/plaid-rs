
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ProcessorTransactionsGetRequestOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_logo_and_counterparty_beta: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_original_description: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_personal_finance_category: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_personal_finance_category_beta: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,
}
impl std::fmt::Display for ProcessorTransactionsGetRequestOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}