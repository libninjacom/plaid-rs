use serde::{Serialize, Deserialize};
///Name information for the associated individual watchlist hit
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct IndividualScreeningHitNames {
    ///The full name of the individual, including all parts.
    pub full: String,
    ///Primary names are those most commonly used to refer to this person. Only one name will ever be marked as primary.
    pub is_primary: bool,
    ///Names that are explicitly marked as low quality either by their `source` list, or by `plaid` by a series of additional checks done by Plaid. Plaid does not ever surface a hit as a result of a weak name alone. If a name has no quality issues, this value will be `none`.
    pub weak_alias_determination: String,
}
impl std::fmt::Display for IndividualScreeningHitNames {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}