use serde::{Serialize, Deserialize};
use super::TransferRefund;
///Defines the response schema for `/transfer/refund/get`
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TransferRefundGetResponse {
    ///Represents a refund within the Transfers API.
    pub refund: TransferRefund,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
}
impl std::fmt::Display for TransferRefundGetResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}