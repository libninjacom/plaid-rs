use serde::{Serialize, Deserialize};
/**A representation of where a transaction took place.

Use this field to pass in structured location information you may have about your transactions. Providing location data is optional but can increase result quality. If you have unstructured location information, it may be appended to the `description` field.*/
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ClientProvidedTransactionLocation {
    ///The street address where the transaction occurred.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    ///The city where the transaction occurred.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    ///The country where the transaction occurred.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    ///The postal code where the transaction occurred.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
    ///The region or state where the transaction occurred.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
}
impl std::fmt::Display for ClientProvidedTransactionLocation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}