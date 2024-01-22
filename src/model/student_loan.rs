use serde::{Serialize, Deserialize};
use super::{PslfStatus, ServicerAddressData, StudentLoanStatus, StudentRepaymentPlan};
///Contains details about a student loan account
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct StudentLoan {
    ///The ID of the account that this liability belongs to.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    ///The account number of the loan. For some institutions, this may be a masked version of the number (e.g., the last 4 digits instead of the entire number).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub account_number: Option<String>,
    ///The dates on which loaned funds were disbursed or will be disbursed. These are often in the past. Dates are returned in an [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format (YYYY-MM-DD).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disbursement_dates: Option<Vec<chrono::NaiveDate>>,
    ///The date when the student loan is expected to be paid off. Availability for this field is limited. Dates are returned in an [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format (YYYY-MM-DD).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expected_payoff_date: Option<chrono::NaiveDate>,
    ///The guarantor of the student loan.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub guarantor: Option<String>,
    ///The interest rate on the loan as a percentage.
    pub interest_rate_percentage: f64,
    ///`true` if a payment is currently overdue. Availability for this field is limited.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_overdue: Option<bool>,
    ///The amount of the last payment.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_payment_amount: Option<f64>,
    ///The date of the last payment. Dates are returned in an [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format (YYYY-MM-DD).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_payment_date: Option<chrono::NaiveDate>,
    ///The date of the last statement. Dates are returned in an [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format (YYYY-MM-DD).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_statement_issue_date: Option<chrono::NaiveDate>,
    ///The type of loan, e.g., "Consolidation Loans".
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub loan_name: Option<String>,
    ///An object representing the status of the student loan
    pub loan_status: StudentLoanStatus,
    /**The minimum payment due for the next billing cycle. There are some exceptions:
Some institutions require a minimum payment across all loans associated with an account number. Our API presents that same minimum payment amount on each loan. The institutions that do this are: Great Lakes ( `ins_116861`), Firstmark (`ins_116295`), Commonbond Firstmark Services (`ins_116950`), Nelnet (`ins_116528`), EdFinancial Services (`ins_116304`), Granite State (`ins_116308`), and Oklahoma Student Loan Authority (`ins_116945`).
Firstmark (`ins_116295` ) and Navient (`ins_116248`) will display as $0 if there is an autopay program in effect.*/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub minimum_payment_amount: Option<f64>,
    ///The due date for the next payment. The due date is `null` if a payment is not expected. A payment is not expected if `loan_status.type` is `deferment`, `in_school`, `consolidated`, `paid in full`, or `transferred`. Dates are returned in an [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format (YYYY-MM-DD).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub next_payment_due_date: Option<chrono::NaiveDate>,
    ///The date on which the loan was initially lent. Dates are returned in an [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format (YYYY-MM-DD).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origination_date: Option<chrono::NaiveDate>,
    ///The original principal balance of the loan.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origination_principal_amount: Option<f64>,
    ///The total dollar amount of the accrued interest balance. For Sallie Mae ( `ins_116944`), this amount is included in the current balance of the loan, so this field will return as `null`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub outstanding_interest_amount: Option<f64>,
    ///The relevant account number that should be used to reference this loan for payments. In the majority of cases, `payment_reference_number` will match `account_number,` but in some institutions, such as Great Lakes (`ins_116861`), it will be different.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub payment_reference_number: Option<String>,
    ///Information about the student's eligibility in the Public Service Loan Forgiveness program. This is only returned if the institution is FedLoan (`ins_116527`).
    pub pslf_status: PslfStatus,
    ///An object representing the repayment plan for the student loan
    pub repayment_plan: StudentRepaymentPlan,
    ///The sequence number of the student loan. Heartland ECSI (`ins_116948`) does not make this field available.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sequence_number: Option<String>,
    ///The address of the student loan servicer. This is generally the remittance address to which payments should be sent.
    pub servicer_address: ServicerAddressData,
    ///The year to date (YTD) interest paid. Availability for this field is limited.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ytd_interest_paid: Option<f64>,
    ///The year to date (YTD) principal paid. Availability for this field is limited.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ytd_principal_paid: Option<f64>,
}
impl std::fmt::Display for StudentLoan {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}