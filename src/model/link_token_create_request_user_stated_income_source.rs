use serde::{Serialize, Deserialize};
///Specifies user stated income sources for the Income product
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LinkTokenCreateRequestUserStatedIncomeSource {
    ///The income category for a specified income source
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    ///The employer corresponding to an income source specified by the user
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub employer: Option<String>,
    ///The income amount paid annually for a specified income source
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pay_annual: Option<f64>,
    ///The pay frequency of a specified income source
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pay_frequency: Option<String>,
    ///The income amount paid per cycle for a specified income source
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pay_per_cycle: Option<f64>,
    ///The pay type - `GROSS`, `NET`, or `UNKNOWN` for a specified income source
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pay_type: Option<String>,
}
impl std::fmt::Display for LinkTokenCreateRequestUserStatedIncomeSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}