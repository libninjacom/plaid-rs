use serde::{Serialize, Deserialize};
use super::StatementsStatement;
///Account associated with the Item.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct StatementsAccount {
    ///Plaid's unique identifier for the account.
    pub account_id: String,
    ///The last 2-4 alphanumeric characters of an account's official account number. Note that the mask may be non-unique between an Item's accounts, and it may also not match the mask that the bank displays to the user.
    pub account_mask: String,
    ///The name of the account, either assigned by the user or by the financial institution itself.
    pub account_name: String,
    ///The official name of the account as given by the financial institution.
    pub account_official_name: String,
    ///The subtype of the account. For a full list of valid types and subtypes, see the [Account schema](https://plaid.com/docs/api/accounts#account-type-schema).
    pub account_subtype: String,
    ///The type of account. For a full list of valid types and subtypes, see the [Account schema](https://plaid.com/docs/api/accounts#account-type-schema).
    pub account_type: String,
    ///The list of statements' metadata associated with this account.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub statements: Vec<StatementsStatement>,
}
impl std::fmt::Display for StatementsAccount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}