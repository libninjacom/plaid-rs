
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ItemApplicationListUserAuth {
    pub fi_username_hash: Option<String>,
    pub user_id: Option<String>,
}
impl std::fmt::Display for ItemApplicationListUserAuth {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}