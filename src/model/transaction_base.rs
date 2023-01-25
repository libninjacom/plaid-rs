
use serde::{Serialize, Deserialize};
use super::{Location, PaymentMeta};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TransactionBase {
    pub account_id: String,
    pub account_owner: Option<String>,
    pub amount: f64,
    pub category: Option<Vec<String>>,
    pub category_id: Option<String>,
    pub check_number: Option<String>,
    pub date: String,
    pub iso_currency_code: Option<String>,
    pub location: Option<Location>,
    pub logo_url: Option<String>,
    pub merchant_name: Option<String>,
    pub name: Option<String>,
    pub original_description: Option<String>,
    pub payment_meta: Option<PaymentMeta>,
    pub pending: bool,
    pub pending_transaction_id: Option<String>,
    pub transaction_id: String,
    pub transaction_type: Option<String>,
    pub unofficial_currency_code: Option<String>,
    pub website: Option<String>,
}
impl std::fmt::Display for TransactionBase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}