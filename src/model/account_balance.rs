
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AccountBalance {
    pub available: Option<f64>,
    pub current: Option<f64>,
    pub iso_currency_code: Option<String>,
    pub last_updated_datetime: Option<String>,
    pub limit: Option<f64>,
    pub unofficial_currency_code: Option<String>,
}
impl std::fmt::Display for AccountBalance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}