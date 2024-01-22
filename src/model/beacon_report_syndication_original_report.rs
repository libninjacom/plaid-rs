use serde::{Serialize, Deserialize};
/**A subset of information from a Beacon Report that has been syndicated to a matching Beacon User in your program.

The `id` field in the response is the ID of the original report that was syndicated. If the original report was created by your organization, the field will be filled with the ID of the report. Otherwise, the field will be `null` indicating that the original report was created by another Beacon customer.*/
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BeaconReportSyndicationOriginalReport {
    ///An ISO8601 formatted timestamp.
    pub created_at: chrono::DateTime<chrono::Utc>,
    ///A date in the format YYYY-MM-DD (RFC 3339 Section 5.6).
    pub fraud_date: chrono::NaiveDate,
    ///ID of the associated Beacon Report.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /**The type of Beacon Report.

`first_party`: If this is the same individual as the one who submitted the KYC.

`stolen`: If this is a different individual from the one who submitted the KYC.

`synthetic`: If this is an individual using fabricated information.

`account_takeover`: If this individual's account was compromised.

`unknown`: If you aren't sure who committed the fraud.*/
    #[serde(rename = "type")]
    pub type_: String,
}
impl std::fmt::Display for BeaconReportSyndicationOriginalReport {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}