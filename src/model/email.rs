use serde::{Serialize, Deserialize};
///An object representing an email address
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Email {
    ///The email address.
    pub data: String,
    ///When `true`, identifies the email address as the primary email on an account.
    pub primary: bool,
    ///The type of email account as described by the financial institution.
    #[serde(rename = "type")]
    pub type_: String,
}
impl std::fmt::Display for Email {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}