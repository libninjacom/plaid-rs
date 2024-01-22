use serde::{Serialize, Deserialize};
///Allows specifying the metadata of the test account
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Meta {
    ///The account's limit
    pub limit: f64,
    ///The account's mask. Should be an empty string or a string of 2-4 alphanumeric characters. This allows you to model a mask which does not match the account number (such as with a virtual account number).
    pub mask: String,
    ///The account's name
    pub name: String,
    ///The account's official name
    pub official_name: String,
}
impl std::fmt::Display for Meta {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}