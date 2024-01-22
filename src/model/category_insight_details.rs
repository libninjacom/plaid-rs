use serde::{Serialize, Deserialize};
///Insights object for categories.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CategoryInsightDetails {
    ///Category name.
    pub name: String,
    ///The most common counterparties associated with this category sorted by outflow.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub top_counterparties: Option<Vec<String>>,
    ///Sum of inflow amounts.
    pub total_inflows: f64,
    ///Sum of outflow amounts.
    pub total_outflows: f64,
    ///The number of transactions associated with this category.
    pub transaction_count: i64,
}
impl std::fmt::Display for CategoryInsightDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}