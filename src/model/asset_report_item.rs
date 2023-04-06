
use serde::{Serialize, Deserialize};
use super::AccountAssets;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetReportItem {
    pub accounts: Vec<AccountAssets>,
    pub date_last_updated: chrono::DateTime<chrono::Utc>,
    pub institution_id: String,
    pub institution_name: String,
    pub item_id: String,
}
impl std::fmt::Display for AssetReportItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}