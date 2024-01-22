use serde::{Serialize, Deserialize};
use super::Owner;
///The Item's bank accounts that have the selected data.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CraBankIncomeAccount {
    /**The last 2-4 alphanumeric characters of an account's official account number.
Note that the mask may be non-unique between an Item's accounts, and it may also not match the mask that the bank displays to the user.*/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mask: Option<String>,
    ///The name of the bank account.
    pub name: String,
    ///The official name of the bank account.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub official_name: Option<String>,
    ///Data returned by the financial institution about the account owner or owners. Identity information is optional, so field may return an empty array.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub owners: Vec<Owner>,
    ///Valid account subtypes for depository accounts. For a list containing descriptions of each subtype, see [Account schemas](https://plaid.com/docs/api/accounts/#StandaloneAccountType-depository).
    pub subtype: String,
    ///The account type. This will always be `depository`.
    #[serde(rename = "type")]
    pub type_: String,
}
impl std::fmt::Display for CraBankIncomeAccount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}