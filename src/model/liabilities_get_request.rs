
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LiabilitiesGetRequest {
    pub access_token: String,
    pub options: Option<LiabilitiesGetRequestOptions>,
}
impl std::fmt::Display for LiabilitiesGetRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}