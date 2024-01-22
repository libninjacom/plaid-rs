use serde::{Serialize, Deserialize};
///Defines the response schema for `/transfer/metrics/get`
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TransferMetricsGetResponse {
    ///Sum of dollar amount of credit transfers in last 24 hours (decimal string with two digits of precision e.g. "10.00").
    pub daily_credit_transfer_volume: String,
    ///Sum of dollar amount of debit transfers in last 24 hours (decimal string with two digits of precision e.g. "10.00").
    pub daily_debit_transfer_volume: String,
    ///The currency of the dollar amount, e.g. "USD".
    pub iso_currency_code: String,
    ///Sum of dollar amount of credit transfers in current calendar month (decimal string with two digits of precision e.g. "10.00").
    pub monthly_credit_transfer_volume: String,
    ///Sum of dollar amount of debit transfers in current calendar month (decimal string with two digits of precision e.g. "10.00").
    pub monthly_debit_transfer_volume: String,
    ///Sum of dollar amount of credit and debit transfers in current calendar month (decimal string with two digits of precision e.g. "10.00").
    pub monthly_transfer_volume: String,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
}
impl std::fmt::Display for TransferMetricsGetResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}