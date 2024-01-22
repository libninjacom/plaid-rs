use serde::{Serialize, Deserialize};
use super::{Address, Email, PhoneNumber};
///Data about the owner or owners of an account. Any fields not specified will be filled in with default Sandbox information.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct OwnerOverride {
    ///Data about the various addresses associated with the account.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub addresses: Vec<Address>,
    ///A list of email addresses associated with the account.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub emails: Vec<Email>,
    ///A list of names associated with the account by the financial institution. These should always be the names of individuals, even for business accounts. Note that the same name data will be used for all accounts associated with an Item.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub names: Vec<String>,
    ///A list of phone numbers associated with the account.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub phone_numbers: Vec<PhoneNumber>,
}
impl std::fmt::Display for OwnerOverride {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}