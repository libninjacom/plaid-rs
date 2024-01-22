use serde::{Serialize, Deserialize};
///Defines the response schema for `/transfer/questionnaire/create`
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TransferQuestionnaireCreateResponse {
    ///Plaid-hosted onboarding URL that you will redirect the end customer to.
    pub onboarding_url: String,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
}
impl std::fmt::Display for TransferQuestionnaireCreateResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}