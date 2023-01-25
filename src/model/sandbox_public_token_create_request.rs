
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SandboxPublicTokenCreateRequest {
    pub initial_products: Vec<String>,
    pub institution_id: String,
    pub options: Option<SandboxPublicTokenCreateRequestOptions>,
    pub user_token: Option<String>,
}
impl std::fmt::Display for SandboxPublicTokenCreateRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}