use serde::{Serialize, Deserialize};
use super::{
    Deductions, Earnings, Employee, EmploymentDetails, IncomeBreakdown, NetPay,
    PayPeriodDetails, PaystubDetails, PaystubEmployer, PaystubYtdDetails,
};
///An object representing data extracted from the end user's paystub.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Paystub {
    ///An object with the deduction information found on a paystub.
    pub deductions: Deductions,
    ///An identifier of the document referenced by the document metadata.
    pub doc_id: String,
    ///An object representing both a breakdown of earnings on a paystub and the total earnings.
    pub earnings: Earnings,
    ///Data about the employee.
    pub employee: Employee,
    ///Information about the employer on the paystub
    pub employer: PaystubEmployer,
    ///An object representing employment details found on a paystub.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub employment_details: Option<EmploymentDetails>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub income_breakdown: Option<Vec<IncomeBreakdown>>,
    ///An object representing information about the net pay amount on the paystub.
    pub net_pay: NetPay,
    ///Details about the pay period.
    pub pay_period_details: PayPeriodDetails,
    ///An object representing details that can be found on the paystub.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub paystub_details: Option<PaystubDetails>,
    ///The amount of income earned year to date, as based on paystub data.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ytd_earnings: Option<PaystubYtdDetails>,
}
impl std::fmt::Display for Paystub {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}