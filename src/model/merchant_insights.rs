
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MerchantInsights {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_id: Option<String>,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub personal_finance_category_detailed: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub personal_finance_category_primary: Option<String>,
    pub total_inflows: f64,
    pub total_outflows: f64,
    pub transaction_count: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub website: Option<String>,
}
impl std::fmt::Display for MerchantInsights {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}