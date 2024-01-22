use serde::{Serialize, Deserialize};
///Information about the device being used to initiate the authorization. These fields are not currently incorporated into the risk check.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TransferAuthorizationDevice {
    ///The IP address of the device being used to initiate the authorization.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
    ///The user agent of the device being used to initiate the authorization.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_agent: Option<String>,
}
impl std::fmt::Display for TransferAuthorizationDevice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}