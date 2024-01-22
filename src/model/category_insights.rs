use serde::{Serialize, Deserialize};
use super::CategoryInsightDetails;
///Insights on a user's top personal finance categories.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CategoryInsights {
    ///List of insights of top detailed personal finance categories ranked by outflow.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub detailed_category_insights: Option<Vec<CategoryInsightDetails>>,
    ///List of insights of top primary personal finance categories ranked by outflow.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub primary_category_insights: Option<Vec<CategoryInsightDetails>>,
}
impl std::fmt::Display for CategoryInsights {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}