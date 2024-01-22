use serde::{Serialize, Deserialize};
///SignalReturnReportResponse defines the response schema for `/signal/return/report`
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SignalReturnReportResponse {
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
}
impl std::fmt::Display for SignalReturnReportResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}