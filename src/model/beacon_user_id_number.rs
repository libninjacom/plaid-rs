use serde::{Serialize, Deserialize};
///The ID number associated with a Beacon User.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BeaconUserIdNumber {
    ///A globally unique and human readable ID type, specific to the country and document category. For more context on this field, see [Hybrid Input Validation](https://plaid.com/docs/identity-verification/hybrid-input-validation).
    #[serde(rename = "type")]
    pub type_: String,
    ///Value of identity document value typed in by user. Alpha-numeric, with all formatting characters stripped.
    pub value: String,
}
impl std::fmt::Display for BeaconUserIdNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}