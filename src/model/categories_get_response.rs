use serde::{Serialize, Deserialize};
use super::Category;
///CategoriesGetResponse defines the response schema for `/categories/get`
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CategoriesGetResponse {
    ///An array of all of the transaction categories used by Plaid.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub categories: Vec<Category>,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
}
impl std::fmt::Display for CategoriesGetResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}