use serde::{Serialize, Deserialize};
use super::{
    IncomeBreakdown, PaystubOverrideEmployee, PaystubOverrideEmployer,
    PaystubOverridePayPeriodDetails,
};
///An object representing data from a paystub.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PaystubOverride {
    ///The employee on the paystub.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub employee: Option<PaystubOverrideEmployee>,
    ///The employer on the paystub.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub employer: Option<PaystubOverrideEmployer>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub income_breakdown: Option<Vec<IncomeBreakdown>>,
    ///Details about the pay period.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pay_period_details: Option<PaystubOverridePayPeriodDetails>,
}
impl std::fmt::Display for PaystubOverride {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}