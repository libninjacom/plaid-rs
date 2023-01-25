
use serde::{Serialize, Deserialize};
use super::{
    IncomeBreakdown, PayPeriodDetails, PaystubOverrideEmployee, PaystubOverrideEmployer,
};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PaystubOverride {
    pub employee: Option<PaystubOverrideEmployee>,
    pub employer: Option<PaystubOverrideEmployer>,
    pub income_breakdown: Option<Vec<IncomeBreakdown>>,
    pub pay_period_details: Option<PayPeriodDetails>,
}
impl std::fmt::Display for PaystubOverride {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}