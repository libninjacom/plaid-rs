use serde::{Serialize, Deserialize};
///IncomeVerificationCreateResponse defines the response schema for `/income/verification/create`.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct IncomeVerificationCreateResponse {
    ///ID of the verification. This ID is persisted throughout the lifetime of the verification.
    pub income_verification_id: String,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
}
impl std::fmt::Display for IncomeVerificationCreateResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}