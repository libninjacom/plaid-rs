use serde::{Serialize, Deserialize};
use super::{BeaconUserAddress, BeaconUserIdNumber, BeaconUserName};
///A Beacon User's data and resulting analysis when checked against duplicate records and the Beacon Fraud Network.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BeaconUserData {
    /**Even if an address has been collected, some fields may be null depending on the region's addressing system. For example:


Addresses from the United Kingdom will not include a region


Addresses from Hong Kong will not include a postal code*/
    pub address: BeaconUserAddress,
    ///A date in the format YYYY-MM-DD (RFC 3339 Section 5.6).
    pub date_of_birth: chrono::NaiveDate,
    ///A valid email address.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email_address: Option<String>,
    ///The ID number associated with a Beacon User.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id_number: Option<BeaconUserIdNumber>,
    ///An IPv4 or IPV6 address.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
    ///The full name for a given Beacon User.
    pub name: BeaconUserName,
    ///A phone number in E.164 format.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,
}
impl std::fmt::Display for BeaconUserData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}