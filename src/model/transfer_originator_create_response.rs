use serde::{Serialize, Deserialize};
///Defines the response schema for `/transfer/originator/create`
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TransferOriginatorCreateResponse {
    ///The company name of the end customer.
    pub company_name: String,
    ///Client ID of the originator. This identifier will be used when creating transfers and should be stored associated with end user information.
    pub originator_client_id: String,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
}
impl std::fmt::Display for TransferOriginatorCreateResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}