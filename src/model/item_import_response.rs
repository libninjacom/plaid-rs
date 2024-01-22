use serde::{Serialize, Deserialize};
///ItemImportResponse defines the response schema for `/item/import`
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ItemImportResponse {
    ///The access token associated with the Item data is being requested for.
    pub access_token: String,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
}
impl std::fmt::Display for ItemImportResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}