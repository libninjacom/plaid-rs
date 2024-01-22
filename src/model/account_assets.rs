use serde::{Serialize, Deserialize};
use super::{
    AccountBase, AssetReportInvestments, AssetReportTransaction, HistoricalBalance,
    Owner, OwnershipType,
};
///Asset information about an account
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AccountAssets {
    ///A single account at a financial institution.
    #[serde(flatten)]
    pub account_base: AccountBase,
    ///The duration of transaction history available for this Item, typically defined as the time since the date of the earliest transaction in that account.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub days_available: Option<f64>,
    ///Calculated data about the historical balances on the account.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub historical_balances: Option<Vec<HistoricalBalance>>,
    ///A transaction within an investment account.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub investments: Option<AssetReportInvestments>,
    ///Data returned by the financial institution about the account owner or owners.For business accounts, the name reported may be either the name of the individual or the name of the business, depending on the institution. Multiple owners on a single account will be represented in the same `owner` object, not in multiple owner objects within the array. In API versions 2018-05-22 and earlier, the `owners` object is not returned, and instead identity information is returned in the top level `identity` object. For more details, see [Plaid API versioning](https://plaid.com/docs/api/versioning/#version-2019-05-29)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owners: Option<Vec<Owner>>,
    /**How an asset is owned.

`association`: Ownership by a corporation, partnership, or unincorporated association, including for-profit and not-for-profit organizations.
`individual`: Ownership by an individual.
`joint`: Joint ownership by multiple parties.
`trust`: Ownership by a revocable or irrevocable trust.*/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ownership_type: Option<OwnershipType>,
    ///Transaction history associated with the account.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transactions: Option<Vec<AssetReportTransaction>>,
}
impl std::fmt::Display for AccountAssets {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for AccountAssets {
    type Target = AccountBase;
    fn deref(&self) -> &Self::Target {
        &self.account_base
    }
}
impl std::ops::DerefMut for AccountAssets {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.account_base
    }
}