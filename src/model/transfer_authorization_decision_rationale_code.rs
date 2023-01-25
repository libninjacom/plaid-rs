
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Serialize, Deserialize)]
pub enum TransferAuthorizationDecisionRationaleCode {
    #[serde(rename = "NSF")]
    Nsf,
    #[serde(rename = "RISK")]
    Risk,
    #[serde(rename = "TRANSFER_LIMIT_REACHED")]
    TransferLimitReached,
    #[serde(rename = "MANUALLY_VERIFIED_ITEM")]
    ManuallyVerifiedItem,
    #[serde(rename = "ITEM_LOGIN_REQUIRED")]
    ItemLoginRequired,
    #[serde(rename = "PAYMENT_PROFILE_LOGIN_REQUIRED")]
    PaymentProfileLoginRequired,
    #[serde(rename = "ERROR")]
    Error,
    #[serde(rename = "MIGRATED_ACCOUNT_ITEM")]
    MigratedAccountItem,
}