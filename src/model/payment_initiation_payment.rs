
use serde::{Serialize, Deserialize};
use super::{
    ExternalPaymentRefundDetails, ExternalPaymentScheduleBase, PaymentAmount,
    PaymentAmountRefunded, PaymentScheme, RecipientBacs,
};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentInitiationPayment {
    pub adjusted_reference: Option<String>,
    pub adjusted_scheme: Option<PaymentScheme>,
    pub amount: PaymentAmount,
    pub amount_refunded: Option<PaymentAmountRefunded>,
    pub bacs: Option<RecipientBacs>,
    pub consent_id: Option<String>,
    pub iban: Option<String>,
    pub last_status_update: String,
    pub payment_id: String,
    pub recipient_id: String,
    pub reference: String,
    pub refund_details: Option<ExternalPaymentRefundDetails>,
    pub refund_ids: Option<Vec<String>>,
    pub schedule: Option<ExternalPaymentScheduleBase>,
    pub scheme: Option<PaymentScheme>,
    pub status: String,
    pub transaction_id: Option<String>,
    pub wallet_id: Option<String>,
}
impl std::fmt::Display for PaymentInitiationPayment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}