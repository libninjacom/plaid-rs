
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct StudentRepaymentPlan {
    pub description: Option<String>,
    #[serde(rename = "type")]
    pub type_: Option<serde_json::Value>,
}
impl std::fmt::Display for StudentRepaymentPlan {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}