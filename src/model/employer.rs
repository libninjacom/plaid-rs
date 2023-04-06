
use serde::{Serialize, Deserialize};
use super::AddressData;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Employer {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<AddressData>,
    pub confidence_score: f64,
    pub employer_id: String,
    pub name: String,
}
impl std::fmt::Display for Employer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}