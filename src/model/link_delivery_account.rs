use serde::{Serialize, Deserialize};
///Information related to account attached to the connected Item
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LinkDeliveryAccount {
    ///If micro-deposit verification is being used, indicates whether the account being verified is a `business` or `personal` account.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub class_type: Option<String>,
    ///The Plaid `account_id`
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    ///The last 2-4 alphanumeric characters of an account's official account number. Note that the mask may be non-unique between an Item's accounts. It may also not match the mask that the bank displays to the user.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mask: Option<String>,
    ///The official account name
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    ///The account subtype. See the [Account schema](https://plaid.com/docs/api/accounts/#account-type-schema) for a full list of possible values
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subtype: Option<String>,
    ///The account type. See the [Account schema](https://plaid.com/docs/api/accounts/#account-type-schema) for a full list of possible values
    #[serde(rename = "type")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    ///Indicates an Item's micro-deposit-based verification status.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub verification_status: Option<String>,
}
impl std::fmt::Display for LinkDeliveryAccount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}