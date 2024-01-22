use serde::{Serialize, Deserialize};
///The amount of income earned year to date, as based on paystub data.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PaystubYtdDetails {
    ///Year-to-date gross earnings.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gross_earnings: Option<f64>,
    ///Year-to-date net (take home) earnings.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub net_earnings: Option<f64>,
}
impl std::fmt::Display for PaystubYtdDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}