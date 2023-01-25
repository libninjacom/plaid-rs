
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct IncomeVerificationTaxformsGetRequest {
    pub access_token: Option<String>,
    pub income_verification_id: Option<String>,
}
impl std::fmt::Display for IncomeVerificationTaxformsGetRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}