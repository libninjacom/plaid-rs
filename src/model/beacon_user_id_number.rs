
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BeaconUserIdNumber {
    #[serde(rename = "type")]
    pub type_: String,
    pub value: String,
}
impl std::fmt::Display for BeaconUserIdNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}