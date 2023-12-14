
use serde::{Serialize, Deserialize};
use super::{BeaconReportSyndicationAnalysis, BeaconReportSyndicationOriginalReport};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BeaconReportSyndication {
    pub analysis: BeaconReportSyndicationAnalysis,
    pub id: String,
    pub report: BeaconReportSyndicationOriginalReport,
}
impl std::fmt::Display for BeaconReportSyndication {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}