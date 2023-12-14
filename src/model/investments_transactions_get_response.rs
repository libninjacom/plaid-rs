
use serde::{Serialize, Deserialize};
use super::{AccountBase, InvestmentTransaction, Item, Security};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvestmentsTransactionsGetResponse {
    pub accounts: Vec<AccountBase>,
    pub investment_transactions: Vec<InvestmentTransaction>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_investments_fallback_item: Option<bool>,
    pub item: Item,
    pub request_id: String,
    pub securities: Vec<Security>,
    pub total_investment_transactions: i64,
}
impl std::fmt::Display for InvestmentsTransactionsGetResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}