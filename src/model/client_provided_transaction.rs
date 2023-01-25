
use serde::{Serialize, Deserialize};
use super::ClientProvidedTransactionLocation;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ClientProvidedTransaction {
    pub amount: f64,
    pub date_posted: Option<String>,
    pub description: String,
    pub direction: String,
    pub id: String,
    pub iso_currency_code: String,
    pub location: Option<ClientProvidedTransactionLocation>,
    pub mcc: Option<String>,
}
impl std::fmt::Display for ClientProvidedTransaction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}