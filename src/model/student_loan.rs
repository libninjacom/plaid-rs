
use serde::{Serialize, Deserialize};
use super::{PslfStatus, ServicerAddressData, StudentLoanStatus, StudentRepaymentPlan};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StudentLoan {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disbursement_dates: Option<Vec<chrono::NaiveDate>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_payoff_date: Option<chrono::NaiveDate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guarantor: Option<String>,
    pub interest_rate_percentage: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_overdue: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_payment_amount: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_payment_date: Option<chrono::NaiveDate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_statement_issue_date: Option<chrono::NaiveDate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loan_name: Option<String>,
    pub loan_status: StudentLoanStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_payment_amount: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_payment_due_date: Option<chrono::NaiveDate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origination_date: Option<chrono::NaiveDate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origination_principal_amount: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outstanding_interest_amount: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_reference_number: Option<String>,
    pub pslf_status: PslfStatus,
    pub repayment_plan: StudentRepaymentPlan,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sequence_number: Option<String>,
    pub servicer_address: ServicerAddressData,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ytd_interest_paid: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ytd_principal_paid: Option<f64>,
}
impl std::fmt::Display for StudentLoan {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}