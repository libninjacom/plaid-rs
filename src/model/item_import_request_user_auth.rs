use serde::{Serialize, Deserialize};
///Object of user ID and auth token pair, permitting Plaid to aggregate a user’s accounts
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ItemImportRequestUserAuth {
    ///Authorization token Plaid will use to aggregate this user’s accounts
    pub auth_token: String,
    ///Opaque user identifier
    pub user_id: String,
}
impl std::fmt::Display for ItemImportRequestUserAuth {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}