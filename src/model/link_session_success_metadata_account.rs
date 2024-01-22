use serde::{Serialize, Deserialize};
///An account attached to the connected Item.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LinkSessionSuccessMetadataAccount {
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
    ///The account subtype. See the [Account schema](/docs/api/accounts#account-type-schema) for a full list of possible values
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subtype: Option<String>,
    ///The account type. See the [Account schema](/docs/api/accounts#account-type-schema) for a full list of possible values
    #[serde(rename = "type")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    /**Indicates an Item's micro-deposit-based verification status. Possible values are:

`pending_automatic_verification`: The Item is pending automatic verification

`pending_manual_verification`: The Item is pending manual micro-deposit verification. Items remain in this state until the user successfully verifies the two amounts.

`automatically_verified`: The Item has successfully been automatically verified

`manually_verified`: The Item has successfully been manually verified

`verification_expired`: Plaid was unable to automatically verify the deposit within 7 calendar days and will no longer attempt to validate the Item. Users may retry by submitting their information again through Link.

`verification_failed`: The Item failed manual micro-deposit verification because the user exhausted all 3 verification attempts. Users may retry by submitting their information again through Link.

`database_matched`: The Item has successfully been verified using Plaid's data sources. Note: Database Match is currently a beta feature, please contact your account manager for more information.

`null`: micro-deposit-based verification is not being used for the Item.*/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub verification_status: Option<String>,
}
impl std::fmt::Display for LinkSessionSuccessMetadataAccount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}