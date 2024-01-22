use serde::{Serialize, Deserialize};
///ProcessorTokenPermissionsGetResponse defines the response schema for `/processor/token/permissions/get`
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ProcessorTokenPermissionsGetResponse {
    ///A list of products the processor token should have access to. An empty list means that the processor has access to all available products, including future products.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub products: Vec<String>,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
}
impl std::fmt::Display for ProcessorTokenPermissionsGetResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}