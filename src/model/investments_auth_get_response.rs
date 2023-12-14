
use serde::{Serialize, Deserialize};
use super::{
    AccountBase, Holding, InvestmentsAuthGetNumbers, InvestmentsAuthOwner, Item, Security,
};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvestmentsAuthGetResponse {
    pub accounts: Vec<AccountBase>,
    pub holdings: Vec<Holding>,
    pub item: Item,
    pub numbers: InvestmentsAuthGetNumbers,
    pub owners: Vec<InvestmentsAuthOwner>,
    pub request_id: String,
    pub securities: Vec<Security>,
}
impl std::fmt::Display for InvestmentsAuthGetResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}