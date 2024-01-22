use serde::{Serialize, Deserialize};
/**An object with keys of `account_id`'s that are mapped to their respective identity attributes that changed.

Example: `{ "XMBvvyMGQ1UoLbKByoMqH3nXMj84ALSdE5B58": ["PHONES"] }`*/
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AccountIdsWithUpdatedIdentity {}
impl std::fmt::Display for AccountIdsWithUpdatedIdentity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}