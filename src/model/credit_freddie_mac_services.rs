use serde::{Serialize, Deserialize};
use super::CreditFreddieMacService;
///A collection of objects that describe requests and responses for services.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreditFreddieMacServices {
    ///A collection of details related to a fulfillment service or product in terms of request, process and result.
    #[serde(rename = "SERVICE")]
    pub service: CreditFreddieMacService,
}
impl std::fmt::Display for CreditFreddieMacServices {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}