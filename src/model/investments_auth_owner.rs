use serde::{Serialize, Deserialize};
///Information on the ownership of an investments account
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct InvestmentsAuthOwner {
    ///The ID of the account that this identity information pertains to
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /**A list of names associated with the account by the financial institution. In the case of a joint account, Plaid will make a best effort to report the names of all account holders.

If an Item contains multiple accounts with different owner names, some institutions will report all names associated with the Item in each account's `names` array.*/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub names: Option<Vec<String>>,
}
impl std::fmt::Display for InvestmentsAuthOwner {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}