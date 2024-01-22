use serde::{Serialize, Deserialize};
///Metadata specifically related to which auth methods an institution supports.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AuthSupportedMethods {
    ///Indicates if automated microdeposits are supported.
    pub automated_micro_deposits: bool,
    ///Indicates if instant auth is supported.
    pub instant_auth: bool,
    ///Indicates if instant match is supported.
    pub instant_match: bool,
    ///Indicates if instant microdeposits are supported.
    pub instant_micro_deposits: bool,
}
impl std::fmt::Display for AuthSupportedMethods {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}