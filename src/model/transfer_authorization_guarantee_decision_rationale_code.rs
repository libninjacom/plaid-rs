
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Serialize, Deserialize)]
pub enum TransferAuthorizationGuaranteeDecisionRationaleCode {
    #[serde(rename = "RETURN_BANK")]
    ReturnBank,
    #[serde(rename = "RETURN_CUSTOMER")]
    ReturnCustomer,
    #[serde(rename = "GUARANTEE_LIMIT_REACHED")]
    GuaranteeLimitReached,
    #[serde(rename = "RISK_ESTIMATE_UNAVAILABLE")]
    RiskEstimateUnavailable,
    #[serde(rename = "REQUIRED_PARAM_MISSING")]
    RequiredParamMissing,
}