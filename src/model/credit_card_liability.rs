use serde::{Serialize, Deserialize};
use super::Apr;
///An object representing a credit card account.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreditCardLiability {
    ///The ID of the account that this liability belongs to.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    ///The various interest rates that apply to the account. APR information is not provided by all card issuers; if APR data is not available, this array will be empty.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub aprs: Vec<Apr>,
    ///true if a payment is currently overdue. Availability for this field is limited.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_overdue: Option<bool>,
    ///The amount of the last payment.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_payment_amount: Option<f64>,
    ///The date of the last payment. Dates are returned in an [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format (YYYY-MM-DD). Availability for this field is limited.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_payment_date: Option<chrono::NaiveDate>,
    ///The total amount owed as of the last statement issued
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_statement_balance: Option<f64>,
    ///The date of the last statement. Dates are returned in an [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format (YYYY-MM-DD).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_statement_issue_date: Option<chrono::NaiveDate>,
    ///The minimum payment due for the next billing cycle.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub minimum_payment_amount: Option<f64>,
    ///The due date for the next payment. The due date is `null` if a payment is not expected. Dates are returned in an [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format (YYYY-MM-DD).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub next_payment_due_date: Option<chrono::NaiveDate>,
}
impl std::fmt::Display for CreditCardLiability {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}