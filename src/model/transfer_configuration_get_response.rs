use serde::{Serialize, Deserialize};
///Defines the response schema for `/transfer/configuration/get`
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TransferConfigurationGetResponse {
    ///The currency of the dollar amount, e.g. "USD".
    pub iso_currency_code: String,
    ///The max limit of sum of dollar amount of credit transfers in last 24 hours (decimal string with two digits of precision e.g. "10.00").
    pub max_daily_credit_amount: String,
    ///The max limit of sum of dollar amount of debit transfers in last 24 hours (decimal string with two digits of precision e.g. "10.00").
    pub max_daily_debit_amount: String,
    ///The max limit of sum of dollar amount of credit and debit transfers in one calendar month (decimal string with two digits of precision e.g. "10.00").
    pub max_monthly_amount: String,
    ///The max limit of sum of dollar amount of credit transfers in one calendar month (decimal string with two digits of precision e.g. "10.00").
    pub max_monthly_credit_amount: String,
    ///The max limit of sum of dollar amount of debit transfers in one calendar month (decimal string with two digits of precision e.g. "10.00").
    pub max_monthly_debit_amount: String,
    ///The max limit of dollar amount of a single transfer (decimal string with two digits of precision e.g. "10.00").
    pub max_single_transfer_amount: String,
    ///The max limit of dollar amount of a single credit transfer (decimal string with two digits of precision e.g. "10.00").
    pub max_single_transfer_credit_amount: String,
    ///The max limit of dollar amount of a single debit transfer (decimal string with two digits of precision e.g. "10.00").
    pub max_single_transfer_debit_amount: String,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
}
impl std::fmt::Display for TransferConfigurationGetResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}