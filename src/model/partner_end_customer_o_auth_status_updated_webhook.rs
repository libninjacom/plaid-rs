use serde::{Serialize, Deserialize};
///The webhook of type `PARTNER` and code `END_CUSTOMER_OAUTH_STATUS_UPDATED` will be fired when a partner's end customer has an update on their OAuth registration status with an institution.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PartnerEndCustomerOAuthStatusUpdatedWebhook {
    ///The client ID of the end customer
    pub end_customer_client_id: String,
    ///The Plaid environment the webhook was sent from
    pub environment: String,
    ///The institution ID
    pub institution_id: String,
    ///The institution name
    pub institution_name: String,
    ///The OAuth status of the update
    pub status: String,
    ///`END_CUSTOMER_OAUTH_STATUS_UPDATED`
    pub webhook_code: String,
    ///`PARTNER`
    pub webhook_type: String,
}
impl std::fmt::Display for PartnerEndCustomerOAuthStatusUpdatedWebhook {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}