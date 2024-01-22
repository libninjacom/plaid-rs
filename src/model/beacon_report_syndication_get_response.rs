use serde::{Serialize, Deserialize};
use super::{BeaconReportSyndicationAnalysis, BeaconReportSyndicationOriginalReport};
/**A Beacon Report Syndication represents a Beacon Report created either by your organization or another Beacon customer that matches a specific Beacon User you've created.

The `analysis` field in the response indicates which fields matched between the originally reported Beacon User and the Beacon User that the report was syndicated to.

The `report` field in the response contains a subset of information from the original report.*/
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BeaconReportSyndicationGetResponse {
    ///Analysis of which fields matched between the originally reported Beacon User and the Beacon User that the report was syndicated to.
    pub analysis: BeaconReportSyndicationAnalysis,
    ///ID of the associated Beacon Report Syndication.
    pub id: String,
    /**A subset of information from a Beacon Report that has been syndicated to a matching Beacon User in your program.

The `id` field in the response is the ID of the original report that was syndicated. If the original report was created by your organization, the field will be filled with the ID of the report. Otherwise, the field will be `null` indicating that the original report was created by another Beacon customer.*/
    pub report: BeaconReportSyndicationOriginalReport,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
}
impl std::fmt::Display for BeaconReportSyndicationGetResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}