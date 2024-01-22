use serde::{Serialize, Deserialize};
///Fired when a Beacon User created within your organization matches one of your existing users.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BeaconDuplicateDetectedWebhook {
    ///The ID of the associated Beacon Duplicate.
    pub beacon_duplicate_id: String,
    ///The Plaid environment the webhook was sent from
    pub environment: String,
    ///`DUPLICATE_DETECTED`
    pub webhook_code: String,
    ///`BEACON`
    pub webhook_type: String,
}
impl std::fmt::Display for BeaconDuplicateDetectedWebhook {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}