use serde::{Serialize, Deserialize};
///An object containing a BACS account number and sort code. If an IBAN is not provided or if you need to accept domestic GBP-denominated payments, BACS data is required.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RecipientBacs {
    ///The account number of the account. Maximum of 10 characters.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub account: Option<String>,
    ///The 6-character sort code of the account.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sort_code: Option<String>,
}
impl std::fmt::Display for RecipientBacs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}