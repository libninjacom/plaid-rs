
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct HistoricalBalance {
    pub current: f64,
    pub date: String,
    pub iso_currency_code: Option<String>,
    pub unofficial_currency_code: Option<String>,
}
impl std::fmt::Display for HistoricalBalance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}