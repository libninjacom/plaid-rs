
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PaymentMeta {
    pub by_order_of: Option<String>,
    pub payee: Option<String>,
    pub payer: Option<String>,
    pub payment_method: Option<String>,
    pub payment_processor: Option<String>,
    pub ppd_id: Option<String>,
    pub reason: Option<String>,
    pub reference_number: Option<String>,
}
impl std::fmt::Display for PaymentMeta {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}