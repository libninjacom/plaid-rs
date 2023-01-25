
use serde::{Serialize, Deserialize};
use super::{Security, AccountBase, Holding, Item};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvestmentsHoldingsGetResponse {
    pub accounts: Vec<AccountBase>,
    pub holdings: Vec<Holding>,
    pub item: Item,
    pub request_id: String,
    pub securities: Vec<Security>,
}
impl std::fmt::Display for InvestmentsHoldingsGetResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}