use serde::{Serialize, Deserialize};
///An optional object to filter or add data to `/asset_report/get` results. If provided, must be non-`null`.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AssetReportGetRequestOptions {
    ///The maximum number of days of history to include in the Asset Report.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub days_to_include: Option<i64>,
}
impl std::fmt::Display for AssetReportGetRequestOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}