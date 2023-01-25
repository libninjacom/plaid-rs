
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct InstitutionsGetByIdRequestOptions {
    pub include_auth_metadata: Option<bool>,
    pub include_optional_metadata: Option<bool>,
    pub include_payment_initiation_metadata: Option<bool>,
    pub include_status: Option<bool>,
}
impl std::fmt::Display for InstitutionsGetByIdRequestOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}