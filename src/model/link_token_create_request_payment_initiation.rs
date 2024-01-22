use serde::{Serialize, Deserialize};
///Specifies options for initializing Link for use with the Payment Initiation (Europe) product. This field is required if `payment_initiation` is included in the `products` array. Either `payment_id` or `consent_id` must be provided.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LinkTokenCreateRequestPaymentInitiation {
    ///The `consent_id` provided by the `/payment_initiation/consent/create` endpoint.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub consent_id: Option<String>,
    ///The `payment_id` provided by the `/payment_initiation/payment/create` endpoint.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub payment_id: Option<String>,
}
impl std::fmt::Display for LinkTokenCreateRequestPaymentInitiation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}