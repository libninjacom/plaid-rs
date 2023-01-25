
use serde::{Serialize, Deserialize};
use super::Apr;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreditCardLiability {
    pub account_id: Option<String>,
    pub aprs: Vec<Apr>,
    pub is_overdue: Option<bool>,
    pub last_payment_amount: Option<f64>,
    pub last_payment_date: Option<String>,
    pub last_statement_balance: Option<f64>,
    pub last_statement_issue_date: Option<String>,
    pub minimum_payment_amount: Option<f64>,
    pub next_payment_due_date: Option<String>,
}
impl std::fmt::Display for CreditCardLiability {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}