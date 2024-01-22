use serde::{Serialize, Deserialize};
///Optional arguments for `/income/verification/create`
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct IncomeVerificationCreateRequestOptions {
    ///An array of access tokens corresponding to the Items that will be cross-referenced with the product data. Plaid will attempt to correlate transaction history from these Items with data from the user's paystub, such as date and amount. If the `transactions` product was not initialized for the Items during Link, it will be initialized after this Link session.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub access_tokens: Option<Vec<String>>,
}
impl std::fmt::Display for IncomeVerificationCreateRequestOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}