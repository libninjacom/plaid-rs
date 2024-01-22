use serde::{Serialize, Deserialize};
///The address of the employee.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PaystubOverrideEmployeeAddress {
    ///The full city name.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    ///The country of the address.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    ///5 digit postal code.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
    /**The region or state
Example: `"NC"`*/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    /**The full street address
Example: `"564 Main Street, APT 15"`*/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub street: Option<String>,
}
impl std::fmt::Display for PaystubOverrideEmployeeAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}