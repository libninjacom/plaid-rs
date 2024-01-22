use serde::{Serialize, Deserialize};
use super::{AccountBase, Holding, Item, Security};
///InvestmentsHoldingsGetResponse defines the response schema for `/investments/holdings/get`
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct InvestmentsHoldingsGetResponse {
    ///The accounts associated with the Item
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub accounts: Vec<AccountBase>,
    ///The holdings belonging to investment accounts associated with the Item. Details of the securities in the holdings are provided in the `securities` field.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub holdings: Vec<Holding>,
    ///When true, this field indicates that the Item's portfolio was manually created with the Investments Fallback flow.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_investments_fallback_item: Option<bool>,
    ///Metadata about the Item.
    pub item: Item,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
    ///Objects describing the securities held in the accounts associated with the Item.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub securities: Vec<Security>,
}
impl std::fmt::Display for InvestmentsHoldingsGetResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}