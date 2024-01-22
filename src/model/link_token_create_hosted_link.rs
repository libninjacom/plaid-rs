use serde::{Serialize, Deserialize};
///Configuration parameters for Hosted Link (beta). Only available for participants in the Hosted Link beta program.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LinkTokenCreateHostedLink {
    ///URI that Hosted Link will redirect to upon completion of the Link flow. This will only occur in Hosted Link sessions, not in other implementation methods.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub completion_redirect_uri: Option<String>,
    /**How Plaid should deliver the Plaid Link session to the customer.
'sms' will deliver via SMS. Must pass `user.phone_number`.
'email' will deliver via email. Must pass `user.email_address`. In the Sandbox environment, this field will be ignored; use the Production or Development environment to test Hosted Link session delivery instead.*/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub delivery_method: Option<String>,
    ///How many seconds the link will be valid for. Must be positive. Cannot be longer than 21 days. The default lifetime is 4 hours.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url_lifetime_seconds: Option<i64>,
}
impl std::fmt::Display for LinkTokenCreateHostedLink {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}