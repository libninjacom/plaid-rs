use serde::{Serialize, Deserialize};
use super::{Pay, TotalCanonicalDescription};
///An object representing both the current pay period and year to date amount for a category.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Total {
    ///Commonly used term to describe the line item.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub canonical_description: Option<TotalCanonicalDescription>,
    ///An object representing a monetary amount.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub current_pay: Option<Pay>,
    ///Text of the line item as printed on the paystub.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    ///An object representing a monetary amount.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ytd_pay: Option<Pay>,
}
impl std::fmt::Display for Total {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}