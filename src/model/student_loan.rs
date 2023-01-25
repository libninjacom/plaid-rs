
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StudentLoan {
    pub account_id: Option<String>,
    pub account_number: Option<String>,
    pub disbursement_dates: Option<Vec<String>>,
    pub expected_payoff_date: Option<String>,
    pub guarantor: Option<String>,
    pub interest_rate_percentage: f64,
    pub is_overdue: Option<bool>,
    pub last_payment_amount: Option<f64>,
    pub last_payment_date: Option<String>,
    pub last_statement_issue_date: Option<String>,
    pub loan_name: Option<String>,
    pub loan_status: StudentLoanStatus,
    pub minimum_payment_amount: Option<f64>,
    pub next_payment_due_date: Option<String>,
    pub origination_date: Option<String>,
    pub origination_principal_amount: Option<f64>,
    pub outstanding_interest_amount: Option<f64>,
    pub payment_reference_number: Option<String>,
    pub pslf_status: PslfStatus,
    pub repayment_plan: StudentRepaymentPlan,
    pub sequence_number: Option<String>,
    pub servicer_address: ServicerAddressData,
    pub ytd_interest_paid: Option<f64>,
    pub ytd_principal_paid: Option<f64>,
}
impl std::fmt::Display for StudentLoan {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}