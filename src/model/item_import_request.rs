
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemImportRequest {
    pub options: Option<ItemImportRequestOptions>,
    pub products: Vec<String>,
    pub user_auth: ItemImportRequestUserAuth,
}
impl std::fmt::Display for ItemImportRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}