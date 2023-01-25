
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ClientProvidedRawTransaction {
    pub amount: f64,
    pub description: String,
    pub id: String,
    pub iso_currency_code: String,
}
impl std::fmt::Display for ClientProvidedRawTransaction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}