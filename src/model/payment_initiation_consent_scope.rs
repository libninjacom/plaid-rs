
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Serialize, Deserialize)]
pub enum PaymentInitiationConsentScope {
    #[serde(rename = "ME_TO_ME")]
    MeToMe,
    #[serde(rename = "EXTERNAL")]
    External,
}