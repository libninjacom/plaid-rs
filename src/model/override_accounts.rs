
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OverrideAccounts {
    pub currency: String,
    pub force_available_balance: f64,
    pub holdings: Option<HoldingsOverride>,
    pub identity: OwnerOverride,
    pub income: Option<IncomeOverride>,
    pub inflow_model: InflowModel,
    pub investment_transactions: Option<InvestmentsTransactionsOverride>,
    pub liability: LiabilityOverride,
    pub meta: Meta,
    pub numbers: Numbers,
    pub starting_balance: f64,
    pub subtype: Option<AccountSubtype>,
    pub transactions: Vec<TransactionOverride>,
    #[serde(rename = "type")]
    pub type_: String,
}
impl std::fmt::Display for OverrideAccounts {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}