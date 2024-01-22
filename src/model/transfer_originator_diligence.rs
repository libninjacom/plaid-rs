use serde::{Serialize, Deserialize};
use super::{
    TransferCreditUsageConfiguration, TransferDebitUsageConfiguration,
    TransferFundingAccount, TransferOriginatorAddress,
};
///The diligence information for the originator.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TransferOriginatorDiligence {
    ///The originator's address.
    pub address: TransferOriginatorAddress,
    ///Specifies the originator's expected usage of credits. For all dollar amounts, use a decimal string with two digits of precision e.g. "10.00". This field is required if the originator is expected to process credit transfers.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub credit_usage_configuration: Option<TransferCreditUsageConfiguration>,
    ///The business name of the originator.
    pub dba: String,
    ///Specifies the originator's expected usage of debits. For all dollar amounts, use a decimal string with two digits of precision e.g. "10.00". This field is required if the originator is expected to process debit transfers.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub debit_usage_configuration: Option<TransferDebitUsageConfiguration>,
    ///The originator's funding account, linked with Plaid Link or `/transfer/migrate_account`.
    pub funding_account: TransferFundingAccount,
    ///The NAICS code of the originator.
    pub naics_code: String,
    ///The tax ID of the originator.
    pub tax_id: String,
    ///The website of the originator.
    pub website: String,
}
impl std::fmt::Display for TransferOriginatorDiligence {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}