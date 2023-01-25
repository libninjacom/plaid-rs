
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Paystub {
    pub deductions: Deductions,
    pub doc_id: String,
    pub earnings: Earnings,
    pub employee: Employee,
    pub employer: PaystubEmployer,
    pub employment_details: Option<EmploymentDetails>,
    pub income_breakdown: Option<Vec<IncomeBreakdown>>,
    pub net_pay: NetPay,
    pub pay_period_details: PayPeriodDetails,
    pub paystub_details: Option<PaystubDetails>,
    pub ytd_earnings: Option<PaystubYtdDetails>,
}
impl std::fmt::Display for Paystub {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}