use serde::{Serialize, Deserialize};
///Fired when institution status meets the conditions configured in the developer dashboard.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct InstitutionStatusAlertWebhook {
    ///The Plaid environment the webhook was sent from
    pub environment: String,
    ///The ID of the associated institution.
    pub institution_id: String,
    ///The global success rate of the institution, calculated based on item add health.
    pub institution_overall_success_rate: f64,
    ///`INSTITUTION_STATUS_ALERT_TRIGGERED`
    pub webhook_code: String,
    ///`DASHBOARD_CONFIGURED_ALERT`
    pub webhook_type: String,
}
impl std::fmt::Display for InstitutionStatusAlertWebhook {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}