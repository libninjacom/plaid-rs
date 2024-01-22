use serde::{Serialize, Deserialize};
use super::Application;
///ApplicationGetResponse defines the response schema for `/application/get`
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ApplicationGetResponse {
    ///Metadata about the application
    pub application: Application,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
}
impl std::fmt::Display for ApplicationGetResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}