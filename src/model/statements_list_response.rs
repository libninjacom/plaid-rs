use serde::{Serialize, Deserialize};
use super::StatementsAccount;
///StatementsListResponse defines the response schema for `/statements/list`
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct StatementsListResponse {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub accounts: Vec<StatementsAccount>,
    ///The Plaid Institution ID associated with the Item.
    pub institution_id: String,
    ///The name of the institution associated with the Item.
    pub institution_name: String,
    ///The Plaid Item ID. The `item_id` is always unique; linking the same account at the same institution twice will result in two Items with different `item_id` values. Like all Plaid identifiers, the `item_id` is case-sensitive.
    pub item_id: String,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
}
impl std::fmt::Display for StatementsListResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}