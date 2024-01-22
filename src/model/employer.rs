use serde::{Serialize, Deserialize};
use super::AddressData;
///Data about the employer.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Employer {
    ///Data about the components comprising an address.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<AddressData>,
    ///A number from 0 to 1 indicating Plaid's level of confidence in the pairing between the employer and the institution (not yet implemented).
    pub confidence_score: f64,
    ///Plaid's unique identifier for the employer.
    pub employer_id: String,
    ///The name of the employer
    pub name: String,
}
impl std::fmt::Display for Employer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}