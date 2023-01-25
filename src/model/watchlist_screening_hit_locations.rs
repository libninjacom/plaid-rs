
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WatchlistScreeningHitLocations {
    pub country: String,
    pub full: String,
}
impl std::fmt::Display for WatchlistScreeningHitLocations {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}