use serde::{Serialize, Deserialize};
///The details of a bank employment verification in Link.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreditSessionBankEmploymentResult {
    ///The Plaid Institution ID associated with the Item.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub institution_id: Option<String>,
    ///The Plaid Item ID. The `item_id` is always unique; linking the same account at the same institution twice will result in two Items with different `item_id` values. Like all Plaid identifiers, the `item_id` is case-sensitive.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub item_id: Option<String>,
    /**Status of the Bank Employment Link session.

`APPROVED`: User has approved and verified their employment.

`NO_EMPLOYMENTS_FOUND`: We attempted, but were unable to find any employment in the connected account.

`EMPLOYER_NOT_LISTED`: The user explicitly indicated that they did not see their current or previous employer in the list of employer names found.

`STARTED`: The user began the bank income portion of the link flow.

`INTERNAL_ERROR`: The user encountered an internal error.*/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}
impl std::fmt::Display for CreditSessionBankEmploymentResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}