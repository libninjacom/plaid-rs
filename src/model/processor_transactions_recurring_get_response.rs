use serde::{Serialize, Deserialize};
use super::TransactionStream;
///ProcessorTransactionsRecurringGetResponse defines the response schema for `/processor/transactions/recurring/get`
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ProcessorTransactionsRecurringGetResponse {
    ///An array of depository transaction streams.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub inflow_streams: Vec<TransactionStream>,
    ///An array of expense transaction streams.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub outflow_streams: Vec<TransactionStream>,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
    ///Timestamp in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format (`YYYY-MM-DDTHH:mm:ssZ`) indicating the last time transaction streams for the given account were updated on
    pub updated_datetime: chrono::DateTime<chrono::Utc>,
}
impl std::fmt::Display for ProcessorTransactionsRecurringGetResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}