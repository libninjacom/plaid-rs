use serde::{Serialize, Deserialize};
use super::{DocumentMetadata, Paystub, PlaidError};
///IncomeVerificationPaystubsGetResponse defines the response schema for `/income/verification/paystubs/get`.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct IncomeVerificationPaystubsGetResponse {
    ///Metadata for an income document.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub document_metadata: Option<Vec<DocumentMetadata>>,
    ///We use standard HTTP response codes for success and failure notifications, and our errors are further classified by `error_type`. In general, 200 HTTP codes correspond to success, 40X codes are for developer- or user-related failures, and 50X codes are for Plaid-related issues. An Item with a non-`null` error object will only be part of an API response when calling `/item/get` to view Item status. Otherwise, error fields will be `null` if no error has occurred; if an error has occurred, an error code will be returned instead.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<PlaidError>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub paystubs: Vec<Paystub>,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
}
impl std::fmt::Display for IncomeVerificationPaystubsGetResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}