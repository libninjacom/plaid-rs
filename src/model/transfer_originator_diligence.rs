
use serde::{Serialize, Deserialize};
use super::{
    TransferCreditUsageConfiguration, TransferDebitUsageConfiguration,
    TransferFundingAccount, TransferOriginatorAddress,
};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferOriginatorDiligence {
    pub address: TransferOriginatorAddress,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credit_usage_configuration: Option<TransferCreditUsageConfiguration>,
    pub dba: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub debit_usage_configuration: Option<TransferDebitUsageConfiguration>,
    pub funding_account: TransferFundingAccount,
    pub naics_code: String,
    pub tax_id: String,
    pub website: String,
}
impl std::fmt::Display for TransferOriginatorDiligence {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}