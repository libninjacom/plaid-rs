
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ItemActivityListRequest {
    pub access_token: Option<String>,
    pub count: Option<i64>,
    pub cursor: Option<String>,
}
impl std::fmt::Display for ItemActivityListRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}