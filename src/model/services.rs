
use serde::{Serialize, Deserialize};
use super::Service;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Services {
    #[serde(rename = "SERVICE")]
    pub service: Service,
}
impl std::fmt::Display for Services {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}