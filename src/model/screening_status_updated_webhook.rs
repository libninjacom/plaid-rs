use serde::{Serialize, Deserialize};
///Fired when an individual screening status has changed, which can occur manually via the dashboard or during ongoing monitoring.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ScreeningStatusUpdatedWebhook {
    ///The Plaid environment the webhook was sent from
    pub environment: String,
    ///The ID of the associated screening.
    pub screening_id: String,
    ///`STATUS_UPDATED`
    pub webhook_code: String,
    ///`SCREENING`
    pub webhook_type: String,
}
impl std::fmt::Display for ScreeningStatusUpdatedWebhook {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}