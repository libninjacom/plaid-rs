
use serde::{Serialize, Deserialize};
use super::{LiabilitiesAccountIdsWithUpdatedLiabilities, PlaidError};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LiabilitiesDefaultUpdateWebhook {
    pub account_ids_with_new_liabilities: Vec<String>,
    pub account_ids_with_updated_liabilities: LiabilitiesAccountIdsWithUpdatedLiabilities,
    pub environment: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<PlaidError>,
    pub item_id: String,
    pub webhook_code: String,
    pub webhook_type: String,
}
impl std::fmt::Display for LiabilitiesDefaultUpdateWebhook {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}