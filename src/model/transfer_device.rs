use serde::{Serialize, Deserialize};
///Information about the device being used to initiate the authorization.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TransferDevice {
    ///The IP address of the device being used to initiate the authorization.
    pub ip_address: String,
    ///The user agent of the device being used to initiate the authorization.
    pub user_agent: String,
}
impl std::fmt::Display for TransferDevice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}