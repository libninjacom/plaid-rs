use serde::{Serialize, Deserialize};
///The legal name and other information for the account holder.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BankTransferUser {
    ///The account holder’s email.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email_address: Option<String>,
    ///The account holder’s full legal name. If the transfer `ach_class` is `ccd`, this should be the business name of the account holder.
    pub legal_name: String,
    ///The account holder's routing number. This field is only used in response data. Do not provide this field when making requests.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub routing_number: Option<String>,
}
impl std::fmt::Display for BankTransferUser {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}