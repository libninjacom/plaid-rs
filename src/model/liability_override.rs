use serde::{Serialize, Deserialize};
use super::{Address, PslfStatus, StudentLoanRepaymentModel, StudentLoanStatus};
///Used to configure Sandbox test data for the Liabilities product
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LiabilityOverride {
    ///The balance transfer APR percentage value. Can only be set if `type` is `credit`.
    pub balance_transfer_apr: f64,
    ///The cash APR percentage value. Can only be set if `type` is `credit`.
    pub cash_apr: f64,
    ///Override the `expected_payoff_date` field. Can only be set if `type` is `student`.
    pub expected_payoff_date: chrono::NaiveDate,
    ///Override the `guarantor` field. Can only be set if `type` is `student`.
    pub guarantor: String,
    ///If set, interest capitalization begins at the given number of months after loan origination. By default interest is never capitalized. Can only be set if `type` is `student`.
    pub interest_capitalization_grace_period_months: f64,
    ///Override the `is_federal` field. Can only be set if `type` is `student`.
    pub is_federal: bool,
    ///Override the `is_overdue` field
    pub is_overdue: bool,
    ///Override the `last_payment_amount` field. Can only be set if `type` is `credit`.
    pub last_payment_amount: f64,
    ///Override the `loan_name` field. Can only be set if `type` is `student`.
    pub loan_name: String,
    ///An object representing the status of the student loan
    pub loan_status: StudentLoanStatus,
    ///Override the `minimum_payment_amount` field. Can only be set if `type` is `credit` or `student`.
    pub minimum_payment_amount: f64,
    ///The interest rate on the loan as a percentage. Can only be set if `type` is `student`.
    pub nominal_apr: f64,
    ///The date on which the loan was initially lent, in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) (YYYY-MM-DD) format. Can only be set if `type` is `student`.
    pub origination_date: chrono::NaiveDate,
    ///Override the `payment_reference_number` field. Can only be set if `type` is `student`.
    pub payment_reference_number: String,
    ///The original loan principal. Can only be set if `type` is `student`.
    pub principal: f64,
    ///Information about the student's eligibility in the Public Service Loan Forgiveness program. This is only returned if the institution is FedLoan (`ins_116527`).
    pub pslf_status: PslfStatus,
    ///The purchase APR percentage value. For simplicity, this is the only interest rate used to calculate interest charges. Can only be set if `type` is `credit`.
    pub purchase_apr: f64,
    ///Student loan repayment information used to configure Sandbox test data for the Liabilities product
    pub repayment_model: StudentLoanRepaymentModel,
    ///Override the `repayment_plan.description` field. Can only be set if `type` is `student`.
    pub repayment_plan_description: String,
    ///Override the `repayment_plan.type` field. Can only be set if `type` is `student`. Possible values are: `"extended graduated"`, `"extended standard"`, `"graduated"`, `"income-contingent repayment"`, `"income-based repayment"`, `"interest only"`, `"other"`, `"pay as you earn"`, `"revised pay as you earn"`, `"standard"`, or `"saving on a valuable education"`.
    pub repayment_plan_type: String,
    ///Override the `sequence_number` field. Can only be set if `type` is `student`.
    pub sequence_number: String,
    ///A physical mailing address.
    pub servicer_address: Address,
    ///The special APR percentage value. Can only be set if `type` is `credit`.
    pub special_apr: f64,
    ///The type of the liability object, either `credit` or `student`. Mortgages are not currently supported in the custom Sandbox.
    #[serde(rename = "type")]
    pub type_: String,
}
impl std::fmt::Display for LiabilityOverride {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}