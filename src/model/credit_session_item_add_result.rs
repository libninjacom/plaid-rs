use serde::{Serialize, Deserialize};
///The details of an Item add in Link.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreditSessionItemAddResult {
    ///The Plaid Institution ID associated with the Item.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub institution_id: Option<String>,
    ///The Plaid Item ID. The `item_id` is always unique; linking the same account at the same institution twice will result in two Items with different `item_id` values. Like all Plaid identifiers, the `item_id` is case-sensitive.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub item_id: Option<String>,
    ///Returned once a user has successfully linked their Item.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub public_token: Option<String>,
}
impl std::fmt::Display for CreditSessionItemAddResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}