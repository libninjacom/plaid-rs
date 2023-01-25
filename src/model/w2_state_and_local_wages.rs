
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct W2StateAndLocalWages {
    pub employer_state_id_number: Option<String>,
    pub local_income_tax: Option<String>,
    pub local_wages_tips: Option<String>,
    pub locality_name: Option<String>,
    pub state: Option<String>,
    pub state_income_tax: Option<String>,
    pub state_wages_tips: Option<String>,
}
impl std::fmt::Display for W2StateAndLocalWages {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}