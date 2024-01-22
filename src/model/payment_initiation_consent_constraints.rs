use serde::{Serialize, Deserialize};
use super::{
    PaymentConsentMaxPaymentAmount, PaymentConsentPeriodicAmount,
    PaymentConsentValidDateTime,
};
///Limitations that will be applied to payments initiated using the payment consent.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PaymentInitiationConsentConstraints {
    ///Maximum amount of a single payment initiated using the payment consent.
    pub max_payment_amount: PaymentConsentMaxPaymentAmount,
    ///A list of amount limitations per period of time.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub periodic_amounts: Vec<PaymentConsentPeriodicAmount>,
    ///Life span for the payment consent. After the `to` date the payment consent expires and can no longer be used for payment initiation.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub valid_date_time: Option<PaymentConsentValidDateTime>,
}
impl std::fmt::Display for PaymentInitiationConsentConstraints {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}