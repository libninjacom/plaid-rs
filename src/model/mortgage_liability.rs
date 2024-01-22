use serde::{Serialize, Deserialize};
use super::{MortgageInterestRate, MortgagePropertyAddress};
///Contains details about a mortgage account.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MortgageLiability {
    ///The ID of the account that this liability belongs to.
    pub account_id: String,
    ///The account number of the loan.
    pub account_number: String,
    ///The current outstanding amount charged for late payment.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub current_late_fee: Option<f64>,
    ///Total amount held in escrow to pay taxes and insurance on behalf of the borrower.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub escrow_balance: Option<f64>,
    ///Indicates whether the borrower has private mortgage insurance in effect.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_pmi: Option<bool>,
    ///Indicates whether the borrower will pay a penalty for early payoff of mortgage.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_prepayment_penalty: Option<bool>,
    ///Object containing metadata about the interest rate for the mortgage.
    pub interest_rate: MortgageInterestRate,
    ///The amount of the last payment.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_payment_amount: Option<f64>,
    ///The date of the last payment. Dates are returned in an [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format (YYYY-MM-DD).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_payment_date: Option<chrono::NaiveDate>,
    ///Full duration of mortgage as at origination (e.g. `10 year`).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub loan_term: Option<String>,
    ///Description of the type of loan, for example `conventional`, `fixed`, or `variable`. This field is provided directly from the loan servicer and does not have an enumerated set of possible values.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub loan_type_description: Option<String>,
    ///Original date on which mortgage is due in full. Dates are returned in an [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format (YYYY-MM-DD).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub maturity_date: Option<chrono::NaiveDate>,
    ///The amount of the next payment.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub next_monthly_payment: Option<f64>,
    ///The due date for the next payment. Dates are returned in an [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format (YYYY-MM-DD).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub next_payment_due_date: Option<chrono::NaiveDate>,
    ///The date on which the loan was initially lent. Dates are returned in an [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format (YYYY-MM-DD).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origination_date: Option<chrono::NaiveDate>,
    ///The original principal balance of the mortgage.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origination_principal_amount: Option<f64>,
    ///Amount of loan (principal + interest) past due for payment.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub past_due_amount: Option<f64>,
    ///Object containing fields describing property address.
    pub property_address: MortgagePropertyAddress,
    ///The year to date (YTD) interest paid.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ytd_interest_paid: Option<f64>,
    ///The YTD principal paid.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ytd_principal_paid: Option<f64>,
}
impl std::fmt::Display for MortgageLiability {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}