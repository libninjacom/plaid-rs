
use serde::{Serialize, Deserialize};
use super::{AccountBase, Item, LiabilitiesObject};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LiabilitiesGetResponse {
    pub accounts: Vec<AccountBase>,
    pub item: Item,
    pub liabilities: LiabilitiesObject,
    pub request_id: String,
}
impl std::fmt::Display for LiabilitiesGetResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}