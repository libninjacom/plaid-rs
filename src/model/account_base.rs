use serde::{Serialize, Deserialize};
use super::{AccountBalance, AccountSubtype};
///A single account at a financial institution.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AccountBase {
    /**Plaid’s unique identifier for the account. This value will not change unless Plaid can't reconcile the account with the data returned by the financial institution. This may occur, for example, when the name of the account changes. If this happens a new `account_id` will be assigned to the account.

The `account_id` can also change if the `access_token` is deleted and the same credentials that were used to generate that `access_token` are used to generate a new `access_token` on a later date. In that case, the new `account_id` will be different from the old `account_id`.

If an account with a specific `account_id` disappears instead of changing, the account is likely closed. Closed accounts are not returned by the Plaid API.

Like all Plaid identifiers, the `account_id` is case sensitive.*/
    pub account_id: String,
    ///A set of fields describing the balance for an account. Balance information may be cached unless the balance object was returned by `/accounts/balance/get`.
    pub balances: AccountBalance,
    ///The last 2-4 alphanumeric characters of an account's official account number. Note that the mask may be non-unique between an Item's accounts, and it may also not match the mask that the bank displays to the user.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mask: Option<String>,
    ///The name of the account, either assigned by the user or by the financial institution itself
    pub name: String,
    ///The official name of the account as given by the financial institution
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub official_name: Option<String>,
    ///A unique and persistent identifier for accounts that can be used to trace multiple instances of the same account across different Items for depository accounts. This is currently only supported for Chase Items.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub persistent_account_id: Option<String>,
    ///See the [Account type schema](https://plaid.com/docs/api/accounts/#account-type-schema) for a full listing of account types and corresponding subtypes.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subtype: Option<AccountSubtype>,
    /**`investment:` Investment account. In API versions 2018-05-22 and earlier, this type is called `brokerage` instead.

`credit:` Credit card

`depository:` Depository account

`loan:` Loan account

`other:` Non-specified account type

See the [Account type schema](https://plaid.com/docs/api/accounts#account-type-schema) for a full listing of account types and corresponding subtypes.*/
    #[serde(rename = "type")]
    pub type_: String,
    /**The current verification status of an Auth Item initiated through Automated or Manual micro-deposits.  Returned for Auth Items only.

`pending_automatic_verification`: The Item is pending automatic verification

`pending_manual_verification`: The Item is pending manual micro-deposit verification. Items remain in this state until the user successfully verifies the micro-deposit.

`automatically_verified`: The Item has successfully been automatically verified	

`manually_verified`: The Item has successfully been manually verified

`verification_expired`: Plaid was unable to automatically verify the deposit within 7 calendar days and will no longer attempt to validate the Item. Users may retry by submitting their information again through Link.

`verification_failed`: The Item failed manual micro-deposit verification because the user exhausted all 3 verification attempts. Users may retry by submitting their information again through Link.

`database_matched`: The Item has successfully been verified using Plaid's data sources. Note: Database Match is currently a beta feature, please contact your account manager for more information.*/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub verification_status: Option<String>,
}
impl std::fmt::Display for AccountBase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}