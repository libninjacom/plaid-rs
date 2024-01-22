use serde::{Serialize, Deserialize};
use super::{
    SandboxPublicTokenCreateRequestOptionsIncomeVerification,
    SandboxPublicTokenCreateRequestOptionsTransactions,
};
///An optional set of options to be used when configuring the Item. If specified, must not be `null`.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SandboxPublicTokenCreateRequestOptions {
    ///A set of parameters for income verification options. This field is required if `income_verification` is included in the `initial_products` array.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub income_verification: Option<
        SandboxPublicTokenCreateRequestOptionsIncomeVerification,
    >,
    ///Test password to use for the creation of the Sandbox Item. Default value is `pass_good`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub override_password: Option<String>,
    ///Test username to use for the creation of the Sandbox Item. Default value is `user_good`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub override_username: Option<String>,
    ///An optional set of parameters corresponding to transactions options.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transactions: Option<SandboxPublicTokenCreateRequestOptionsTransactions>,
    ///Specify a webhook to associate with the new Item.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub webhook: Option<String>,
}
impl std::fmt::Display for SandboxPublicTokenCreateRequestOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}