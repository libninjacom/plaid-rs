use serde::{Serialize, Deserialize};
///Contains the supported service types in RTP
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TransferCapabilitiesGetRtp {
    ///When `true`, the linked Item's institution supports RTP credit transfer.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub credit: Option<bool>,
}
impl std::fmt::Display for TransferCapabilitiesGetRtp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}