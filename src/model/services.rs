use serde::{Serialize, Deserialize};
use super::Service;
///A collection of objects that describe requests and responses for services.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Services {
    ///A collection of details related to a fulfillment service or product in terms of request, process and result.
    #[serde(rename = "SERVICE")]
    pub service: Service,
}
impl std::fmt::Display for Services {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}