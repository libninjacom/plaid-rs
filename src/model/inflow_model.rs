
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct InflowModel {
    pub income_amount: f64,
    pub payment_day_of_month: f64,
    pub statement_day_of_month: String,
    pub transaction_name: String,
    #[serde(rename = "type")]
    pub type_: String,
}
impl std::fmt::Display for InflowModel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}