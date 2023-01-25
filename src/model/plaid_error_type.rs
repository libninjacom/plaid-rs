
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Serialize, Deserialize)]
pub enum PlaidErrorType {
    #[serde(rename = "INVALID_REQUEST")]
    InvalidRequest,
    #[serde(rename = "INVALID_RESULT")]
    InvalidResult,
    #[serde(rename = "INVALID_INPUT")]
    InvalidInput,
    #[serde(rename = "INSTITUTION_ERROR")]
    InstitutionError,
    #[serde(rename = "RATE_LIMIT_EXCEEDED")]
    RateLimitExceeded,
    #[serde(rename = "API_ERROR")]
    ApiError,
    #[serde(rename = "ITEM_ERROR")]
    ItemError,
    #[serde(rename = "ASSET_REPORT_ERROR")]
    AssetReportError,
    #[serde(rename = "RECAPTCHA_ERROR")]
    RecaptchaError,
    #[serde(rename = "OAUTH_ERROR")]
    OauthError,
    #[serde(rename = "PAYMENT_ERROR")]
    PaymentError,
    #[serde(rename = "BANK_TRANSFER_ERROR")]
    BankTransferError,
    #[serde(rename = "INCOME_VERIFICATION_ERROR")]
    IncomeVerificationError,
    #[serde(rename = "MICRODEPOSITS_ERROR")]
    MicrodepositsError,
}