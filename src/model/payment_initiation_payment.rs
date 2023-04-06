
use serde::{Serialize, Deserialize};
use super::{
    ExternalPaymentRefundDetails, ExternalPaymentScheduleBase, PaymentAmount,
    PaymentAmountRefunded, PaymentScheme, RecipientBacs,
};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentInitiationPayment {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adjusted_reference: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adjusted_scheme: Option<PaymentScheme>,
    pub amount: PaymentAmount,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_refunded: Option<PaymentAmountRefunded>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bacs: Option<RecipientBacs>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consent_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iban: Option<String>,
    pub last_status_update: chrono::DateTime<chrono::Utc>,
    pub payment_id: String,
    pub recipient_id: String,
    pub reference: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refund_details: Option<ExternalPaymentRefundDetails>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refund_ids: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule: Option<ExternalPaymentScheduleBase>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheme: Option<PaymentScheme>,
    pub status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wallet_id: Option<String>,
}
impl std::fmt::Display for PaymentInitiationPayment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}