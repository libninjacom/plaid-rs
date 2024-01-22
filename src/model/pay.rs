use serde::{Serialize, Deserialize};
///An object representing a monetary amount.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Pay {
    ///A numerical amount of a specific currency.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub amount: Option<f64>,
    ///Currency code, e.g. USD
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
}
impl std::fmt::Display for Pay {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}