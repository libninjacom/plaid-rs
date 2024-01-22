use serde::{Serialize, Deserialize};
///The full name for a given Beacon User.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BeaconUserName {
    ///A string with at least one non-whitespace character, with a max length of 100 characters.
    pub family_name: String,
    ///A string with at least one non-whitespace character, with a max length of 100 characters.
    pub given_name: String,
}
impl std::fmt::Display for BeaconUserName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}