use serde::{Serialize, Deserialize};
use super::InstitutionSupportedNetworks;
///Defines the response schema for `/transfer/capabilities/get`
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TransferCapabilitiesGetResponse {
    ///Contains the RTP network and types supported by the linked Item's institution.
    pub institution_supported_networks: InstitutionSupportedNetworks,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
}
impl std::fmt::Display for TransferCapabilitiesGetResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}