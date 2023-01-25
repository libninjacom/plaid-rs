
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SignalPersonName {
    pub family_name: Option<String>,
    pub given_name: Option<String>,
    pub middle_name: Option<String>,
    pub prefix: Option<String>,
    pub suffix: Option<String>,
}
impl std::fmt::Display for SignalPersonName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}