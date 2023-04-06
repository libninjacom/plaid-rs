
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferSweep {
    pub amount: String,
    pub created: chrono::DateTime<chrono::Utc>,
    pub funding_account_id: String,
    pub id: String,
    pub iso_currency_code: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settled: Option<chrono::NaiveDate>,
}
impl std::fmt::Display for TransferSweep {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}