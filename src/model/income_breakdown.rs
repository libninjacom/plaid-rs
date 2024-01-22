use serde::{Serialize, Deserialize};
use super::IncomeBreakdownType;
///An object representing a breakdown of the different income types on the paystub.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct IncomeBreakdown {
    ///The number of hours logged for this income for this pay period.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hours: Option<f64>,
    ///The hourly rate at which the income is paid.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rate: Option<f64>,
    ///The total pay for this pay period.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total: Option<f64>,
    /**The type of income. Possible values include:
  `"regular"`: regular income
  `"overtime"`: overtime income
  `"bonus"`: bonus income*/
    #[serde(rename = "type")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<IncomeBreakdownType>,
}
impl std::fmt::Display for IncomeBreakdown {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}