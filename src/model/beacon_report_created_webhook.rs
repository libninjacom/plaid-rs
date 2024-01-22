use serde::{Serialize, Deserialize};
///Fired when one of your Beacon Users is first reported to the Beacon network.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BeaconReportCreatedWebhook {
    ///The ID of the associated Beacon Report.
    pub beacon_report_id: String,
    ///The Plaid environment the webhook was sent from
    pub environment: String,
    ///`REPORT_CREATED`
    pub webhook_code: String,
    ///`BEACON`
    pub webhook_type: String,
}
impl std::fmt::Display for BeaconReportCreatedWebhook {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}