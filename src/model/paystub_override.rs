
use serde::{Serialize, Deserialize};
use super::{
    IncomeBreakdown, PayPeriodDetails, PaystubOverrideEmployee, PaystubOverrideEmployer,
};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PaystubOverride {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub employee: Option<PaystubOverrideEmployee>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub employer: Option<PaystubOverrideEmployer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub income_breakdown: Option<Vec<IncomeBreakdown>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pay_period_details: Option<PayPeriodDetails>,
}
impl std::fmt::Display for PaystubOverride {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}