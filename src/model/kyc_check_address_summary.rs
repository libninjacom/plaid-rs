
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct KycCheckAddressSummary {
    pub po_box: String,
    pub summary: String,
    #[serde(rename = "type")]
    pub type_: String,
}
impl std::fmt::Display for KycCheckAddressSummary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}