
use serde::{Serialize, Deserialize};
use super::{Address, PslfStatus, StudentLoanRepaymentModel, StudentLoanStatus};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LiabilityOverride {
    pub balance_transfer_apr: f64,
    pub cash_apr: f64,
    pub expected_payoff_date: chrono::NaiveDate,
    pub guarantor: String,
    pub interest_capitalization_grace_period_months: f64,
    pub is_federal: bool,
    pub is_overdue: bool,
    pub last_payment_amount: f64,
    pub loan_name: String,
    pub loan_status: StudentLoanStatus,
    pub minimum_payment_amount: f64,
    pub nominal_apr: f64,
    pub origination_date: chrono::NaiveDate,
    pub payment_reference_number: String,
    pub principal: f64,
    pub pslf_status: PslfStatus,
    pub purchase_apr: f64,
    pub repayment_model: StudentLoanRepaymentModel,
    pub repayment_plan_description: String,
    pub repayment_plan_type: String,
    pub sequence_number: String,
    pub servicer_address: Address,
    pub special_apr: f64,
    #[serde(rename = "type")]
    pub type_: String,
}
impl std::fmt::Display for LiabilityOverride {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}