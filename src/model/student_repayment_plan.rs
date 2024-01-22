use serde::{Serialize, Deserialize};
///An object representing the repayment plan for the student loan
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct StudentRepaymentPlan {
    ///The description of the repayment plan as provided by the servicer.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    ///The type of the repayment plan.
    #[serde(rename = "type")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<serde_json::Value>,
}
impl std::fmt::Display for StudentRepaymentPlan {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}