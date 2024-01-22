use serde::{Serialize, Deserialize};
///Specifies options for initializing Link for use with the Transfer product.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LinkTokenCreateRequestTransfer {
    ///The `id` returned by the `/transfer/intent/create` endpoint.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub intent_id: Option<String>,
    ///The `payment_profile_token` returned by the `/payment_profile/create` endpoint.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub payment_profile_token: Option<String>,
}
impl std::fmt::Display for LinkTokenCreateRequestTransfer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}