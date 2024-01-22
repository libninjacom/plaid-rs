use serde::{Serialize, Deserialize};
///A representation of where a transaction took place
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Location {
    ///The street address where the transaction occurred.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    ///The city where the transaction occurred.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    ///The ISO 3166-1 alpha-2 country code where the transaction occurred.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    ///The latitude where the transaction occurred.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lat: Option<f64>,
    ///The longitude where the transaction occurred.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lon: Option<f64>,
    ///The postal code where the transaction occurred. In API versions 2018-05-22 and earlier, this field is called `zip`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
    ///The region or state where the transaction occurred. In API versions 2018-05-22 and earlier, this field is called `state`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    ///The merchant defined store number where the transaction occurred.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub store_number: Option<String>,
}
impl std::fmt::Display for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}