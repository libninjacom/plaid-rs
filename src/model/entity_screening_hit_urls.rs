use serde::{Serialize, Deserialize};
///URLs associated with the entity screening hit
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EntityScreeningHitUrls {
    ///An 'http' or 'https' URL (must begin with either of those).
    pub url: String,
}
impl std::fmt::Display for EntityScreeningHitUrls {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}