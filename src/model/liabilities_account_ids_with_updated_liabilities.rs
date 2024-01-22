use serde::{Serialize, Deserialize};
/**An object with keys of `account_id`'s that are mapped to their respective liabilities fields that changed.

Example: `{ "XMBvvyMGQ1UoLbKByoMqH3nXMj84ALSdE5B58": ["past_amount_due"] }`*/
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LiabilitiesAccountIdsWithUpdatedLiabilities {}
impl std::fmt::Display for LiabilitiesAccountIdsWithUpdatedLiabilities {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}