
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MortgageLiability {
    pub account_id: String,
    pub account_number: String,
    pub current_late_fee: Option<f64>,
    pub escrow_balance: Option<f64>,
    pub has_pmi: Option<bool>,
    pub has_prepayment_penalty: Option<bool>,
    pub interest_rate: MortgageInterestRate,
    pub last_payment_amount: Option<f64>,
    pub last_payment_date: Option<String>,
    pub loan_term: Option<String>,
    pub loan_type_description: Option<String>,
    pub maturity_date: Option<String>,
    pub next_monthly_payment: Option<f64>,
    pub next_payment_due_date: Option<String>,
    pub origination_date: Option<String>,
    pub origination_principal_amount: Option<f64>,
    pub past_due_amount: Option<f64>,
    pub property_address: MortgagePropertyAddress,
    pub ytd_interest_paid: Option<f64>,
    pub ytd_principal_paid: Option<f64>,
}
impl std::fmt::Display for MortgageLiability {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}