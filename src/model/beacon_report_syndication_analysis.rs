
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BeaconReportSyndicationAnalysis {
    pub address: String,
    pub date_of_birth: String,
    pub email_address: String,
    pub id_number: String,
    pub ip_address: String,
    pub name: String,
    pub phone_number: String,
}
impl std::fmt::Display for BeaconReportSyndicationAnalysis {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}