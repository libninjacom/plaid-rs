
use serde::{Serialize, Deserialize};
use super::{
    AccountBase, AssetReportInvestments, AssetReportTransaction, HistoricalBalance,
    Owner, OwnershipType,
};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountAssets {
    #[serde(flatten)]
    pub account_base: AccountBase,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub days_available: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub historical_balances: Option<Vec<HistoricalBalance>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub investments: Option<AssetReportInvestments>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owners: Option<Vec<Owner>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ownership_type: Option<OwnershipType>,
    #[serde(skip_serializing_if = "Option::is_none")]
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