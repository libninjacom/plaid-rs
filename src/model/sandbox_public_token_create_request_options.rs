
use serde::{Serialize, Deserialize};
use super::{
    SandboxPublicTokenCreateRequestOptionsIncomeVerification,
    SandboxPublicTokenCreateRequestOptionsTransactions,
};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SandboxPublicTokenCreateRequestOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub income_verification: Option<
        SandboxPublicTokenCreateRequestOptionsIncomeVerification,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub override_password: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub override_username: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transactions: Option<SandboxPublicTokenCreateRequestOptionsTransactions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub webhook: Option<String>,
}
impl std::fmt::Display for SandboxPublicTokenCreateRequestOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}