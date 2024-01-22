use serde::{Serialize, Deserialize};
///Location information for the associated individual watchlist hit
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WatchlistScreeningHitLocations {
    ///Valid, capitalized, two-letter ISO code representing the country of this object. Must be in ISO 3166-1 alpha-2 form.
    pub country: String,
    ///The full location string, potentially including elements like street, city, postal codes and country codes. Note that this is not necessarily a complete or well-formatted address.
    pub full: String,
}
impl std::fmt::Display for WatchlistScreeningHitLocations {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}