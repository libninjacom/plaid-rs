
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UserAddress {
    pub city: String,
    pub country: String,
    pub postal_code: String,
    pub region: String,
    pub street: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub street2: Option<String>,
}
impl std::fmt::Display for UserAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}