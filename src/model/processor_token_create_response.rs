use serde::{Serialize, Deserialize};
///ProcessorTokenCreateResponse defines the response schema for `/processor/token/create` and `/processor/apex/processor_token/create`
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ProcessorTokenCreateResponse {
    ///The `processor_token` that can then be used by the Plaid partner to make API requests
    pub processor_token: String,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
}
impl std::fmt::Display for ProcessorTokenCreateResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}