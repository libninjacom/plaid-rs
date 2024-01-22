use serde::{Serialize, Deserialize};
use super::{
    AccountBase, Holding, InvestmentsAuthGetNumbers, InvestmentsAuthOwner, Item, Security,
};
///InvestmentsAuthGetResponse defines the response schema for `/investments/auth/get`
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct InvestmentsAuthGetResponse {
    ///The accounts for which data is being retrieved
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub accounts: Vec<AccountBase>,
    ///The holdings belonging to investment accounts associated with the Item. Details of the securities in the holdings are provided in the `securities` field.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub holdings: Vec<Holding>,
    ///Metadata about the Item.
    pub item: Item,
    ///Identifying information for transferring holdings to an investments account.
    pub numbers: InvestmentsAuthGetNumbers,
    ///Information about the account owners for the accounts associated with the Item.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub owners: Vec<InvestmentsAuthOwner>,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
    ///Objects describing the securities held in the accounts associated with the Item.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub securities: Vec<Security>,
}
impl std::fmt::Display for InvestmentsAuthGetResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}