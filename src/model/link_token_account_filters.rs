use serde::{Serialize, Deserialize};
use super::{CreditFilter, DepositoryFilter, InvestmentFilter, LoanFilter, OtherFilter};
/**By default, Link will provide limited account filtering: it will only display Institutions that are compatible with all products supplied in the `products` parameter of `/link/token/create`, and, if `auth` is specified in the `products` array, will also filter out accounts other than `checking` and `savings` accounts on the Account Select pane. You can further limit the accounts shown in Link by using `account_filters` to specify the account subtypes to be shown in Link. Only the specified subtypes will be shown. This filtering applies to both the Account Select view (if enabled) and the Institution Select view. Institutions that do not support the selected subtypes will be omitted from Link. To indicate that all subtypes should be shown, use the value `"all"`. If the `account_filters` filter is used, any account type for which a filter is not specified will be entirely omitted from Link. For a full list of valid types and subtypes, see the [Account schema](https://plaid.com/docs/api/accounts#account-type-schema).

The filter may or may not impact the list of accounts shown by the institution in the OAuth account selection flow, depending on the specific institution. If the user selects excluded account subtypes in the OAuth flow, these accounts will not be added to the Item. If the user selects only excluded account subtypes, the link attempt will fail and the user will be prompted to try again.*/
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LinkTokenAccountFilters {
    ///A filter to apply to `credit`-type accounts
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub credit: Option<CreditFilter>,
    ///A filter to apply to `depository`-type accounts
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub depository: Option<DepositoryFilter>,
    ///A filter to apply to `investment`-type accounts (or `brokerage`-type accounts for API versions 2018-05-22 and earlier).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub investment: Option<InvestmentFilter>,
    ///A filter to apply to `loan`-type accounts
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub loan: Option<LoanFilter>,
    ///A filter to apply to `other`-type accounts
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub other: Option<OtherFilter>,
}
impl std::fmt::Display for LinkTokenAccountFilters {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}