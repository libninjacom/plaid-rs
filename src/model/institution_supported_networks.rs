
use serde::{Serialize, Deserialize};
use super::TransferCapabilitiesGetRtp;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstitutionSupportedNetworks {
    pub rtp: TransferCapabilitiesGetRtp,
}
impl std::fmt::Display for InstitutionSupportedNetworks {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}