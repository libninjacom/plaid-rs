use serde::{Serialize, Deserialize};
///Fired when a Beacon User status has changed, which can occur manually via the dashboard or when information is reported to the Beacon network.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BeaconUserStatusUpdatedWebhook {
    ///The ID of the associated Beacon user.
    pub beacon_user_id: String,
    ///The Plaid environment the webhook was sent from
    pub environment: String,
    ///`USER_STATUS_UPDATED`
    pub webhook_code: String,
    ///`BEACON`
    pub webhook_type: String,
}
impl std::fmt::Display for BeaconUserStatusUpdatedWebhook {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}