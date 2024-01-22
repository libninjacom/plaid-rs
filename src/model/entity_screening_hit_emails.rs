use serde::{Serialize, Deserialize};
///Email address information for the associated entity watchlist hit
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EntityScreeningHitEmails {
    ///A valid email address.
    pub email_address: String,
}
impl std::fmt::Display for EntityScreeningHitEmails {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}