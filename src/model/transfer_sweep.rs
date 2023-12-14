
use serde::{Serialize, Deserialize};
use super::SweepStatus;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferSweep {
    pub amount: String,
    pub created: chrono::DateTime<chrono::Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub funding_account_id: String,
    pub id: String,
    pub iso_currency_code: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_trace_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settled: Option<chrono::NaiveDate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<SweepStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger: Option<String>,
}
impl std::fmt::Display for TransferSweep {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}