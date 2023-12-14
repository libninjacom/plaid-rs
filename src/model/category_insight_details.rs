
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CategoryInsightDetails {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_counterparties: Option<Vec<String>>,
    pub total_inflows: f64,
    pub total_outflows: f64,
    pub transaction_count: i64,
}
impl std::fmt::Display for CategoryInsightDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}