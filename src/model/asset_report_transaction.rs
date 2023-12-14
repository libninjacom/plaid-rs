
use serde::{Serialize, Deserialize};
use super::{CreditCategory, Location, PaymentMeta};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetReportTransaction {
    pub account_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_owner: Option<String>,
    pub amount: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub check_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credit_category: Option<CreditCategory>,
    pub date: chrono::NaiveDate,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_transacted: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub income_source_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iso_currency_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<Location>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merchant_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub original_description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_meta: Option<PaymentMeta>,
    pub pending: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending_transaction_id: Option<String>,
    pub transaction_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unofficial_currency_code: Option<String>,
}
impl std::fmt::Display for AssetReportTransaction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}