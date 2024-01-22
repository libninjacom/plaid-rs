use serde::{Serialize, Deserialize};
///Identifying information for transferring money to or from a US account via ACH or wire transfer.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NumbersAch {
    /**The ACH account number for the account.

Note that when using OAuth with Chase Bank (`ins_56`), Chase will issue "tokenized" routing and account numbers, which are not the user's actual account and routing numbers. These tokenized account numbers (also known as TANs) should work identically to normal account and routing numbers. The digits returned in the `mask` field will continue to reflect the actual account number, rather than the tokenized account number; for this reason, when displaying account numbers to the user to help them identify their account in your UI, always use the `mask` rather than truncating the `account` number. If a user revokes their permissions to your app, the tokenized numbers will no longer work.*/
    pub account: String,
    ///The Plaid account ID associated with the account numbers
    pub account_id: String,
    ///Whether the account supports ACH transfers into the account
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub can_transfer_in: Option<bool>,
    ///Whether the account supports ACH transfers out of the account
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub can_transfer_out: Option<bool>,
    ///The ACH routing number for the account. If the institution is `ins_56`, this may be a tokenized routing number. For more information, see the description of the `account` field.
    pub routing: String,
    ///The wire transfer routing number for the account, if available
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub wire_routing: Option<String>,
}
impl std::fmt::Display for NumbersAch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}