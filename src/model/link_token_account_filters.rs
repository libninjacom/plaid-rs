
use serde::{Serialize, Deserialize};
use super::{CreditFilter, DepositoryFilter, InvestmentFilter, LoanFilter, OtherFilter};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LinkTokenAccountFilters {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credit: Option<CreditFilter>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub depository: Option<DepositoryFilter>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub investment: Option<InvestmentFilter>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loan: Option<LoanFilter>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub other: Option<OtherFilter>,
}
impl std::fmt::Display for LinkTokenAccountFilters {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}