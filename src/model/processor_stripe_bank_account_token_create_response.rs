use serde::{Serialize, Deserialize};
///ProcessorStripeBankAccountTokenCreateResponse defines the response schema for `/processor/stripe/bank_account/create`
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ProcessorStripeBankAccountTokenCreateResponse {
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
    ///A token that can be sent to Stripe for use in making API calls to Plaid
    pub stripe_bank_account_token: String,
}
impl std::fmt::Display for ProcessorStripeBankAccountTokenCreateResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}