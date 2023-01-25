
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Serialize, Deserialize)]
pub enum PaymentInitiationPaymentStatus {
    #[serde(rename = "PAYMENT_STATUS_INPUT_NEEDED")]
    PaymentStatusInputNeeded,
    #[serde(rename = "PAYMENT_STATUS_PROCESSING")]
    PaymentStatusProcessing,
    #[serde(rename = "PAYMENT_STATUS_INITIATED")]
    PaymentStatusInitiated,
    #[serde(rename = "PAYMENT_STATUS_COMPLETED")]
    PaymentStatusCompleted,
    #[serde(rename = "PAYMENT_STATUS_INSUFFICIENT_FUNDS")]
    PaymentStatusInsufficientFunds,
    #[serde(rename = "PAYMENT_STATUS_FAILED")]
    PaymentStatusFailed,
    #[serde(rename = "PAYMENT_STATUS_BLOCKED")]
    PaymentStatusBlocked,
    #[serde(rename = "PAYMENT_STATUS_UNKNOWN")]
    PaymentStatusUnknown,
    #[serde(rename = "PAYMENT_STATUS_EXECUTED")]
    PaymentStatusExecuted,
    #[serde(rename = "PAYMENT_STATUS_SETTLED")]
    PaymentStatusSettled,
    #[serde(rename = "PAYMENT_STATUS_AUTHORISING")]
    PaymentStatusAuthorising,
    #[serde(rename = "PAYMENT_STATUS_CANCELLED")]
    PaymentStatusCancelled,
    #[serde(rename = "PAYMENT_STATUS_ESTABLISHED")]
    PaymentStatusEstablished,
    #[serde(rename = "PAYMENT_STATUS_REJECTED")]
    PaymentStatusRejected,
}