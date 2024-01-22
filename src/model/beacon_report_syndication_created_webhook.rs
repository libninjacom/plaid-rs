use serde::{Serialize, Deserialize};
///Fired when a report created on the Beacon Network matches with one of your Beacon Users.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BeaconReportSyndicationCreatedWebhook {
    ///The ID of the associated Beacon Report Syndication.
    pub beacon_report_syndication_id: String,
    ///The Plaid environment the webhook was sent from
    pub environment: String,
    ///`REPORT_SYNDICATION_CREATED`
    pub webhook_code: String,
    ///`BEACON`
    pub webhook_type: String,
}
impl std::fmt::Display for BeaconReportSyndicationCreatedWebhook {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}