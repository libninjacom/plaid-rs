
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PaymentMeta {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub by_order_of: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payee: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_processor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ppd_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_number: Option<String>,
}
impl std::fmt::Display for PaymentMeta {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}