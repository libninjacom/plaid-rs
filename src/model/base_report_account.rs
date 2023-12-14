
use serde::{Serialize, Deserialize};
use super::{
    AccountSubtype, BaseReportAccountBalances, BaseReportAccountInsights,
    BaseReportTransaction, HistoricalBalance, Owner, OwnershipType,
};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BaseReportAccount {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_insights: Option<BaseReportAccountInsights>,
    pub balances: BaseReportAccountBalances,
    pub days_available: f64,
    pub historical_balances: Vec<HistoricalBalance>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mask: Option<String>,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub official_name: Option<String>,
    pub owners: Vec<Owner>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ownership_type: Option<OwnershipType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subtype: Option<AccountSubtype>,
    pub transactions: Vec<BaseReportTransaction>,
    #[serde(rename = "type")]
    pub type_: String,
}
impl std::fmt::Display for BaseReportAccount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}