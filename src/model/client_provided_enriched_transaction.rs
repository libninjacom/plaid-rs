
use serde::{Serialize, Deserialize};
use super::Enrichments;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientProvidedEnrichedTransaction {
    pub amount: f64,
    pub description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction: Option<String>,
    pub enrichments: Enrichments,
    pub id: String,
    pub iso_currency_code: String,
}
impl std::fmt::Display for ClientProvidedEnrichedTransaction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}