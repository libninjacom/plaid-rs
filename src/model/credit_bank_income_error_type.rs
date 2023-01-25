
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Serialize, Deserialize)]
pub enum CreditBankIncomeErrorType {
    #[serde(rename = "INTERNAL_SERVER_ERROR")]
    InternalServerError,
    #[serde(rename = "INSUFFICIENT_CREDENTIALS")]
    InsufficientCredentials,
    #[serde(rename = "ITEM_LOCKED")]
    ItemLocked,
    #[serde(rename = "USER_SETUP_REQUIRED")]
    UserSetupRequired,
    #[serde(rename = "COUNTRY_NOT_SUPPORTED")]
    CountryNotSupported,
    #[serde(rename = "INSTITUTION_DOWN")]
    InstitutionDown,
    #[serde(rename = "INSTITUTION_NO_LONGER_SUPPORTED")]
    InstitutionNoLongerSupported,
    #[serde(rename = "INSTITUTION_NOT_RESPONDING")]
    InstitutionNotResponding,
    #[serde(rename = "INVALID_CREDENTIALS")]
    InvalidCredentials,
    #[serde(rename = "INVALID_MFA")]
    InvalidMfa,
    #[serde(rename = "INVALID_SEND_METHOD")]
    InvalidSendMethod,
    #[serde(rename = "ITEM_LOGIN_REQUIRED")]
    ItemLoginRequired,
    #[serde(rename = "MFA_NOT_SUPPORTED")]
    MfaNotSupported,
    #[serde(rename = "NO_ACCOUNTS")]
    NoAccounts,
    #[serde(rename = "ITEM_NOT_SUPPORTED")]
    ItemNotSupported,
    #[serde(rename = "ACCESS_NOT_GRANTED")]
    AccessNotGranted,
}