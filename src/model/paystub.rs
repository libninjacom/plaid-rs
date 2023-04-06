
use serde::{Serialize, Deserialize};
use super::{
    Deductions, Earnings, Employee, EmploymentDetails, IncomeBreakdown, NetPay,
    PayPeriodDetails, PaystubDetails, PaystubEmployer, PaystubYtdDetails,
};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Paystub {
    pub deductions: Deductions,
    pub doc_id: String,
    pub earnings: Earnings,
    pub employee: Employee,
    pub employer: PaystubEmployer,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub employment_details: Option<EmploymentDetails>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub income_breakdown: Option<Vec<IncomeBreakdown>>,
    pub net_pay: NetPay,
    pub pay_period_details: PayPeriodDetails,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paystub_details: Option<PaystubDetails>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ytd_earnings: Option<PaystubYtdDetails>,
}
impl std::fmt::Display for Paystub {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}