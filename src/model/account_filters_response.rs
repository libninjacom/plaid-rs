use serde::{Serialize, Deserialize};
use super::{CreditFilter, DepositoryFilter, InvestmentFilter, LoanFilter};
///The `account_filters` specified in the original call to `/link/token/create`.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AccountFiltersResponse {
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
}
impl std::fmt::Display for AccountFiltersResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}