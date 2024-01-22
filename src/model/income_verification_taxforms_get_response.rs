use serde::{Serialize, Deserialize};
use super::{DocumentMetadata, PlaidError, Taxform};
///IncomeVerificationTaxformsGetResponse defines the response schema for `/income/verification/taxforms/get`
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct IncomeVerificationTaxformsGetResponse {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub document_metadata: Vec<DocumentMetadata>,
    ///We use standard HTTP response codes for success and failure notifications, and our errors are further classified by `error_type`. In general, 200 HTTP codes correspond to success, 40X codes are for developer- or user-related failures, and 50X codes are for Plaid-related issues. An Item with a non-`null` error object will only be part of an API response when calling `/item/get` to view Item status. Otherwise, error fields will be `null` if no error has occurred; if an error has occurred, an error code will be returned instead.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<PlaidError>,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    ///A list of forms.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub taxforms: Vec<Taxform>,
}
impl std::fmt::Display for IncomeVerificationTaxformsGetResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}