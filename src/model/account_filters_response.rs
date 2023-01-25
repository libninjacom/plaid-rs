
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AccountFiltersResponse {
    pub credit: Option<CreditFilter>,
    pub depository: Option<DepositoryFilter>,
    pub investment: Option<InvestmentFilter>,
    pub loan: Option<LoanFilter>,
}
impl std::fmt::Display for AccountFiltersResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}