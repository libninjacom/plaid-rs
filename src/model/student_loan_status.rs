
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct StudentLoanStatus {
    pub end_date: Option<String>,
    #[serde(rename = "type")]
    pub type_: Option<String>,
}
impl std::fmt::Display for StudentLoanStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}