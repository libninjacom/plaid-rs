
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AssetDetail {
    #[serde(rename = "AssetAccountIdentifier")]
    pub asset_account_identifier: String,
    #[serde(rename = "AssetAsOfDate")]
    pub asset_as_of_date: String,
    #[serde(rename = "AssetAvailableBalanceAmount")]
    pub asset_available_balance_amount: f64,
    #[serde(rename = "AssetCurrentBalanceAmount")]
    pub asset_current_balance_amount: f64,
    #[serde(rename = "AssetDaysRequestedCount")]
    pub asset_days_requested_count: i64,
    #[serde(rename = "AssetDescription")]
    pub asset_description: Option<String>,
    #[serde(rename = "AssetOwnershipType")]
    pub asset_ownership_type: Option<String>,
    #[serde(rename = "AssetType")]
    pub asset_type: String,
    #[serde(rename = "AssetTypeAdditionalDescription")]
    pub asset_type_additional_description: Option<String>,
    #[serde(rename = "AssetUniqueIdentifier")]
    pub asset_unique_identifier: String,
}
impl std::fmt::Display for AssetDetail {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}