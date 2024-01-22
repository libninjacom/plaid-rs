use serde::{Serialize, Deserialize};
use super::{AccountBase, IdentityDocument, Owner};
///Identity information about an account
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AccountIdentity {
    ///A single account at a financial institution.
    #[serde(flatten)]
    pub account_base: AccountBase,
    ///Array of documents that identity data is dervied from. This array will be empty if none of the account identity is from a document.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub documents: Option<Vec<IdentityDocument>>,
    ///Data returned by the financial institution about the account owner or owners. Only returned by Identity or Assets endpoints. For business accounts, the name reported may be either the name of the individual or the name of the business, depending on the institution; detecting whether the linked account is a business account is not currently supported. Multiple owners on a single account will be represented in the same `owner` object, not in multiple owner objects within the array. In API versions 2018-05-22 and earlier, the `owners` object is not returned, and instead identity information is returned in the top level `identity` object. For more details, see [Plaid API versioning](https://plaid.com/docs/api/versioning/#version-2019-05-29)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub owners: Vec<Owner>,
}
impl std::fmt::Display for AccountIdentity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for AccountIdentity {
    type Target = AccountBase;
    fn deref(&self) -> &Self::Target {
        &self.account_base
    }
}
impl std::ops::DerefMut for AccountIdentity {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.account_base
    }
}