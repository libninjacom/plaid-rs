use serde::{Serialize, Deserialize};
///An optional object to be used with the request. If specified, `options` must not be `null`.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TransactionsRecurringGetRequestOptions {
    ///Personal finance categories are now returned by default.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub include_personal_finance_category: Option<bool>,
}
impl std::fmt::Display for TransactionsRecurringGetRequestOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}