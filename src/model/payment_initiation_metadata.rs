use serde::{Serialize, Deserialize};
use super::{
    PaymentInitiationMaximumPaymentAmount, PaymentInitiationStandingOrderMetadata,
};
///Metadata that captures what specific payment configurations an institution supports when making Payment Initiation requests.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PaymentInitiationMetadata {
    /**A mapping of currency to maximum payment amount (denominated in the smallest unit of currency) supported by the institution.

Example: `{"GBP": "10000"}`*/
    pub maximum_payment_amount: PaymentInitiationMaximumPaymentAmount,
    ///Metadata specifically related to valid Payment Initiation standing order configurations for the institution.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub standing_order_metadata: Option<PaymentInitiationStandingOrderMetadata>,
    ///Indicates whether the institution supports payments from a different country.
    pub supports_international_payments: bool,
    ///Indicates whether the institution supports returning refund details when initiating a payment.
    pub supports_refund_details: bool,
    ///Indicates whether the institution supports SEPA Instant payments.
    pub supports_sepa_instant: bool,
}
impl std::fmt::Display for PaymentInitiationMetadata {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}