
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Serialize, Deserialize)]
pub enum SignalPaymentMethod {
    #[serde(rename = "SAME_DAY_ACH")]
    SameDayAch,
    #[serde(rename = "NEXT_DAY_ACH")]
    NextDayAch,
    #[serde(rename = "STANDARD_ACH")]
    StandardAch,
    #[serde(rename = "REAL_TIME_PAYMENTS")]
    RealTimePayments,
    #[serde(rename = "DEBIT_CARD")]
    DebitCard,
    #[serde(rename = "MULTIPLE_PAYMENT_METHODS")]
    MultiplePaymentMethods,
}