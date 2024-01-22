use serde::{Serialize, Deserialize};
///You can use this field to pre-populate the user's legal name; if it is provided here, they will not be prompted to enter their name in the identity verification attempt.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct IdentityVerificationRequestUserName {
    ///A string with at least one non-whitespace character, with a max length of 100 characters.
    pub family_name: String,
    ///A string with at least one non-whitespace character, with a max length of 100 characters.
    pub given_name: String,
}
impl std::fmt::Display for IdentityVerificationRequestUserName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}