use serde::{Serialize, Deserialize};
use super::{AccountBase, InvestmentTransaction, Item, Security};
///InvestmentsTransactionsGetResponse defines the response schema for `/investments/transactions/get`
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct InvestmentsTransactionsGetResponse {
    ///The accounts for which transaction history is being fetched.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub accounts: Vec<AccountBase>,
    ///The transactions being fetched
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub investment_transactions: Vec<InvestmentTransaction>,
    ///When true, this field indicates that the Item's portfolio was manually created with the Investments Fallback flow.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_investments_fallback_item: Option<bool>,
    ///Metadata about the Item.
    pub item: Item,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
    ///All securities for which there is a corresponding transaction being fetched.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub securities: Vec<Security>,
    ///The total number of transactions available within the date range specified. If `total_investment_transactions` is larger than the size of the `transactions` array, more transactions are available and can be fetched via manipulating the `offset` parameter.
    pub total_investment_transactions: i64,
}
impl std::fmt::Display for InvestmentsTransactionsGetResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}