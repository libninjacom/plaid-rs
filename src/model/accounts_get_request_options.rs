use serde::{Serialize, Deserialize};
///An optional object to filter `/accounts/get` results.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AccountsGetRequestOptions {
    ///An array of `account_ids` to retrieve for the Account.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<Vec<String>>,
}
impl std::fmt::Display for AccountsGetRequestOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}