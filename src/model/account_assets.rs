
use serde::{Serialize, Deserialize};
use super::{
    AccountBase, AssetReportTransaction, HistoricalBalance, Owner, OwnershipType,
};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountAssets {
    #[serde(flatten)]
    pub account_base: AccountBase,
    pub days_available: f64,
    pub historical_balances: Vec<HistoricalBalance>,
    pub owners: Vec<Owner>,
    pub ownership_type: Option<OwnershipType>,
    pub transactions: Vec<AssetReportTransaction>,
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