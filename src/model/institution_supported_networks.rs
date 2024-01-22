use serde::{Serialize, Deserialize};
use super::TransferCapabilitiesGetRtp;
///Contains the RTP network and types supported by the linked Item's institution.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct InstitutionSupportedNetworks {
    ///Contains the supported service types in RTP
    pub rtp: TransferCapabilitiesGetRtp,
}
impl std::fmt::Display for InstitutionSupportedNetworks {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}