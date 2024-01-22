use serde::{Serialize, Deserialize};
///An optional set of options to be used when configuring the Item. If specified, must not be `null`.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SandboxProcessorTokenCreateRequestOptions {
    ///Test password to use for the creation of the Sandbox Item. Default value is `pass_good`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub override_password: Option<String>,
    ///Test username to use for the creation of the Sandbox Item. Default value is `user_good`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub override_username: Option<String>,
}
impl std::fmt::Display for SandboxProcessorTokenCreateRequestOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}