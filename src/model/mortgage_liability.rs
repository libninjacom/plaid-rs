
use serde::{Serialize, Deserialize};
use super::{MortgageInterestRate, MortgagePropertyAddress};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MortgageLiability {
    pub account_id: String,
    pub account_number: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_late_fee: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub escrow_balance: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_pmi: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_prepayment_penalty: Option<bool>,
    pub interest_rate: MortgageInterestRate,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_payment_amount: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_payment_date: Option<chrono::NaiveDate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loan_term: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loan_type_description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maturity_date: Option<chrono::NaiveDate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_monthly_payment: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_payment_due_date: Option<chrono::NaiveDate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origination_date: Option<chrono::NaiveDate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origination_principal_amount: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub past_due_amount: Option<f64>,
    pub property_address: MortgagePropertyAddress,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ytd_interest_paid: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ytd_principal_paid: Option<f64>,
}
impl std::fmt::Display for MortgageLiability {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}