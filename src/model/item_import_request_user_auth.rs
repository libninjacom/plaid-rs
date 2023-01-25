
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ItemImportRequestUserAuth {
    pub auth_token: String,
    pub user_id: String,
}
impl std::fmt::Display for ItemImportRequestUserAuth {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}