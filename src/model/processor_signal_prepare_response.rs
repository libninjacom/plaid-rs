use serde::{Serialize, Deserialize};
///ProcessorSignalPrepareResponse defines the response schema for `/processor/signal/prepare`
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ProcessorSignalPrepareResponse {
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
}
impl std::fmt::Display for ProcessorSignalPrepareResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}