use serde::{Serialize, Deserialize};
use super::{
    AccountSubtype, BaseReportAccountBalances, BaseReportAccountInsights,
    BaseReportTransaction, HistoricalBalance, Owner, OwnershipType,
};
///Base Report information about an account
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BaseReportAccount {
    ///Calculated insights derived from transaction-level data.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub account_insights: Option<BaseReportAccountInsights>,
    ///Base Report information about an account's balances
    pub balances: BaseReportAccountBalances,
    ///The duration of transaction history available for this Item, typically defined as the time since the date of the earliest transaction in that account. Only returned by Base Report endpoints.
    pub days_available: f64,
    ///Calculated data about the historical balances on the account. Only returned by Base Report endpoints and currently not supported by `brokerage` or `investment` accounts.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub historical_balances: Vec<HistoricalBalance>,
    ///The last 2-4 alphanumeric characters of an account's official account number. Note that the mask may be non-unique between an Item's accounts, and it may also not match the mask that the bank displays to the user.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mask: Option<String>,
    ///The name of the account, either assigned by the user or by the financial institution itself
    pub name: String,
    ///The official name of the account as given by the financial institution
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub official_name: Option<String>,
    ///Data returned by the financial institution about the account owner or owners. For business accounts, the name reported may be either the name of the individual or the name of the business, depending on the institution. Multiple owners on a single account will be represented in the same `owner` object, not in multiple owner objects within the array.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub owners: Vec<Owner>,
    /**How an asset is owned.

`association`: Ownership by a corporation, partnership, or unincorporated association, including for-profit and not-for-profit organizations.
`individual`: Ownership by an individual.
`joint`: Joint ownership by multiple parties.
`trust`: Ownership by a revocable or irrevocable trust.*/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ownership_type: Option<OwnershipType>,
    ///See the [Account type schema](https://plaid.com/docs/api/accounts/#account-type-schema) for a full listing of account types and corresponding subtypes.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subtype: Option<AccountSubtype>,
    ///Transaction history associated with the account. Only returned by Base Report endpoints. Transaction history returned by endpoints such as `/transactions/get` or `/investments/transactions/get` will be returned in the top-level `transactions` field instead.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub transactions: Vec<BaseReportTransaction>,
    /**`investment:` Investment account. In API versions 2018-05-22 and earlier, this type is called `brokerage` instead.

`credit:` Credit card

`depository:` Depository account

`loan:` Loan account

`other:` Non-specified account type

See the [Account type schema](https://plaid.com/docs/api/accounts#account-type-schema) for a full listing of account types and corresponding subtypes.*/
    #[serde(rename = "type")]
    pub type_: String,
}
impl std::fmt::Display for BaseReportAccount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}