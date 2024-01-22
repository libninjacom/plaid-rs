use serde::{Serialize, Deserialize};
///An optional object to filter `/institutions/get` results.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct InstitutionsGetRequestOptions {
    ///When `true`, returns metadata related to the Auth product indicating which auth methods are supported.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub include_auth_metadata: Option<bool>,
    /**When `true`, return the institution's homepage URL, logo and primary brand color.

Note that Plaid does not own any of the logos shared by the API, and that by accessing or using these logos, you agree that you are doing so at your own risk and will, if necessary, obtain all required permissions from the appropriate rights holders and adhere to any applicable usage guidelines. Plaid disclaims all express or implied warranties with respect to the logos.*/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub include_optional_metadata: Option<bool>,
    ///When `true`, returns metadata related to the Payment Initiation product indicating which payment configurations are supported.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub include_payment_initiation_metadata: Option<bool>,
    ///Limit results to institutions with or without OAuth login flows. Note that institutions will have `oauth` set to `true` if some Items associated with that institution are required to use OAuth flows; institutions in a state of migration to OAuth will have the `oauth` attribute set to `true`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub oauth: Option<bool>,
    ///Filter the Institutions based on which products they support.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub products: Option<Vec<String>>,
    ///Specify an array of routing numbers to filter institutions. The response will only return institutions that match all of the routing numbers in the array. Routing number records used for this matching are not comprehensive; failure to match a given routing number to an institution does not mean that the institution is unsupported by Plaid.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub routing_numbers: Option<Vec<String>>,
}
impl std::fmt::Display for InstitutionsGetRequestOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}