
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct JwkPublicKey {
    pub alg: String,
    pub created_at: i64,
    pub crv: String,
    pub expired_at: Option<i64>,
    pub kid: String,
    pub kty: String,
    #[serde(rename = "use")]
    pub use_: String,
    pub x: String,
    pub y: String,
}
impl std::fmt::Display for JwkPublicKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}