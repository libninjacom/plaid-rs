
use serde::{Serialize, Deserialize};
use super::CategoryInsightDetails;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CategoryInsights {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detailed_category_insights: Option<Vec<CategoryInsightDetails>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_category_insights: Option<Vec<CategoryInsightDetails>>,
}
impl std::fmt::Display for CategoryInsights {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}