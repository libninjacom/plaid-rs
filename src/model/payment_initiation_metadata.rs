
use serde::{Serialize, Deserialize};
use super::{
    PaymentInitiationMaximumPaymentAmount, PaymentInitiationStandingOrderMetadata,
};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentInitiationMetadata {
    pub maximum_payment_amount: PaymentInitiationMaximumPaymentAmount,
    pub standing_order_metadata: Option<PaymentInitiationStandingOrderMetadata>,
    pub supports_international_payments: bool,
    pub supports_refund_details: bool,
    pub supports_sepa_instant: bool,
}
impl std::fmt::Display for PaymentInitiationMetadata {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}