
use serde::{Serialize, Deserialize};
use super::Apr;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreditCardLiability {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    pub aprs: Vec<Apr>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_overdue: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_payment_amount: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_payment_date: Option<chrono::NaiveDate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_statement_balance: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_statement_issue_date: Option<chrono::NaiveDate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_payment_amount: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_payment_due_date: Option<chrono::NaiveDate>,
}
impl std::fmt::Display for CreditCardLiability {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}