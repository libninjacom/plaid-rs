use serde::{Serialize, Deserialize};
///Specifies options for initializing Link for use with the Auth product. This field can be used to enable or disable extended Auth flows for the resulting Link session. Omitting any field will result in a default that can be configured by your account manager.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LinkTokenCreateRequestAuth {
    ///Specifies whether Auth Type Select is enabled for the Link session, allowing the end user to choose between linking instantly or manually prior to selecting their financial institution. Note that this can only be true if `same_day_microdeposits_enabled` is set to true.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub auth_type_select_enabled: Option<bool>,
    ///Specifies whether the Link session is enabled for the Automated Micro-deposits flow.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub automated_microdeposits_enabled: Option<bool>,
    ///Specifies whether the Link session is enabled for the Database Match flow.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub database_match_enabled: Option<bool>,
    ///This field has been deprecated in favor of `auth_type_select_enabled`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flow_type: Option<String>,
    ///Specifies whether the Link session is enabled for the Instant Match flow. As of November 2022, Instant Match will be enabled by default. Instant Match can be disabled by setting this field to `false`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub instant_match_enabled: Option<bool>,
    ///Specifies whether the Link session is enabled for the Instant Micro-deposits flow.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub instant_microdeposits_enabled: Option<bool>,
    ///Specifies what type of [Reroute to Credentials](https://plaid.com/docs/auth/coverage/same-day/#reroute-to-credentials) pane should be used in the Link session for the Same Day Micro-deposits flow. As of October 15 2023, the default setting is `OPTIONAL`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reroute_to_credentials: Option<String>,
    ///Specifies whether the Link session is enabled for the Same Day Micro-deposits flow.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub same_day_microdeposits_enabled: Option<bool>,
}
impl std::fmt::Display for LinkTokenCreateRequestAuth {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}