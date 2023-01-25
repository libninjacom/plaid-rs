
use serde::{Serialize, Deserialize};
use super::InstitutionSupportedNetworks;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferCapabilitiesGetResponse {
    pub institution_supported_networks: InstitutionSupportedNetworks,
    pub request_id: String,
}
impl std::fmt::Display for TransferCapabilitiesGetResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}