use serde::{Serialize, Deserialize};
use super::{Address, Email, PhoneNumber};
///Data returned from the financial institution about the owner or owners of an account. Only the `names` array must be non-empty.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Owner {
    ///Data about the various addresses associated with the account by the financial institution. May be an empty array if no relevant information is returned from the financial institution.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub addresses: Vec<Address>,
    ///document_id is the id of the document that this identity belongs to
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub document_id: Option<String>,
    ///A list of email addresses associated with the account by the financial institution. May be an empty array if no relevant information is returned from the financial institution.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub emails: Vec<Email>,
    /**A list of names associated with the account by the financial institution. In the case of a joint account, Plaid will make a best effort to report the names of all account holders.

If an Item contains multiple accounts with different owner names, some institutions will report all names associated with the Item in each account's `names` array.*/
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub names: Vec<String>,
    ///A list of phone numbers associated with the account by the financial institution. May be an empty array if no relevant information is returned from the financial institution.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub phone_numbers: Vec<PhoneNumber>,
}
impl std::fmt::Display for Owner {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}