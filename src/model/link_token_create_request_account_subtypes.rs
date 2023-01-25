
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LinkTokenCreateRequestAccountSubtypes {
    pub credit: Option<LinkTokenCreateCreditFilter>,
    pub depository: Option<LinkTokenCreateDepositoryFilter>,
    pub investment: Option<LinkTokenCreateInvestmentFilter>,
    pub loan: Option<LinkTokenCreateLoanFilter>,
}
impl std::fmt::Display for LinkTokenCreateRequestAccountSubtypes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}