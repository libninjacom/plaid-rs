
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct W2StateAndLocalWages {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub employer_state_id_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_income_tax: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_wages_tips: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locality_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_income_tax: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_wages_tips: Option<String>,
}
impl std::fmt::Display for W2StateAndLocalWages {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}