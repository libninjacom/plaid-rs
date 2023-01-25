
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DocumentaryVerification {
    pub documents: Vec<DocumentaryVerificationDocument>,
    pub status: String,
}
impl std::fmt::Display for DocumentaryVerification {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}