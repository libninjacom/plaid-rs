
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SandboxProcessorTokenCreateRequest {
    pub institution_id: String,
    pub options: Option<SandboxProcessorTokenCreateRequestOptions>,
}
impl std::fmt::Display for SandboxProcessorTokenCreateRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}