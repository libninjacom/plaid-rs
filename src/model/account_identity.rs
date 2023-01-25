
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountIdentity {
    #[serde(flatten)]
    pub account_base: AccountBase,
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