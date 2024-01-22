use serde::{Serialize, Deserialize};
///The full name provided by the user. If the user has not submitted their name, this field will be null. Otherwise, both fields are guaranteed to be filled.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct IdentityVerificationResponseUserName {
    ///A string with at least one non-whitespace character, with a max length of 100 characters.
    pub family_name: String,
    ///A string with at least one non-whitespace character, with a max length of 100 characters.
    pub given_name: String,
}
impl std::fmt::Display for IdentityVerificationResponseUserName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}