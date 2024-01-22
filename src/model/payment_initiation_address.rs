use serde::{Serialize, Deserialize};
///The optional address of the payment recipient's bank account. Required by most institutions outside of the UK.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PaymentInitiationAddress {
    ///The city where the recipient is located. Maximum of 35 characters.
    pub city: String,
    ///The ISO 3166-1 alpha-2 country code where the recipient is located.
    pub country: String,
    ///The postal code where the recipient is located. Maximum of 16 characters.
    pub postal_code: String,
    ///An array of length 1-2 representing the street address where the recipient is located. Maximum of 70 characters.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub street: Vec<String>,
}
impl std::fmt::Display for PaymentInitiationAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}