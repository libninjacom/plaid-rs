use serde::{Serialize, Deserialize};
///The details of a bank income verification in Link
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreditSessionBankIncomeResult {
    ///The Plaid Institution ID associated with the Item.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub institution_id: Option<String>,
    ///The Plaid Item ID. The `item_id` is always unique; linking the same account at the same institution twice will result in two Items with different `item_id` values. Like all Plaid identifiers, the `item_id` is case-sensitive.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub item_id: Option<String>,
    /**Status of the Bank Income Link session.

`APPROVED`: User has approved and verified their income

`NO_DEPOSITS_FOUND`: We attempted, but were unable to find any income in the connected account.

`USER_REPORTED_NO_INCOME`: The user explicitly indicated that they don't receive income in the connected account.

`STARTED`: The user began the bank income portion of the link flow.

`INTERNAL_ERROR`: The user encountered an internal error.*/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}
impl std::fmt::Display for CreditSessionBankIncomeResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}