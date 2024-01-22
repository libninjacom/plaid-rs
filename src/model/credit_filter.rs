use serde::{Serialize, Deserialize};
///A filter to apply to `credit`-type accounts
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreditFilter {
    ///An array of account subtypes to display in Link. If not specified, all account subtypes will be shown. For a full list of valid types and subtypes, see the [Account schema](https://plaid.com/docs/api/accounts#account-type-schema).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub account_subtypes: Vec<String>,
}
impl std::fmt::Display for CreditFilter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}