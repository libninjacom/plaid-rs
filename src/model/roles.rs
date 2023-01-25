
use serde::{Serialize, Deserialize};
use super::Role;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Roles {
    #[serde(rename = "ROLE")]
    pub role: Role,
}
impl std::fmt::Display for Roles {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}