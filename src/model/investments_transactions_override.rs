
use serde::{Serialize, Deserialize};
use super::SecurityOverride;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvestmentsTransactionsOverride {
    pub currency: String,
    pub date: chrono::NaiveDate,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fees: Option<f64>,
    pub name: String,
    pub price: f64,
    pub quantity: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security: Option<SecurityOverride>,
    #[serde(rename = "type")]
    pub type_: String,
}
impl std::fmt::Display for InvestmentsTransactionsOverride {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}