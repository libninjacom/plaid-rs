
use serde::{Serialize, Deserialize};
use super::{AuthMetadata, InstitutionStatus, PaymentInitiationMetadata};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Institution {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_metadata: Option<AuthMetadata>,
    pub country_codes: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dtc_numbers: Option<Vec<String>>,
    pub institution_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logo: Option<String>,
    pub name: String,
    pub oauth: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_initiation_metadata: Option<PaymentInitiationMetadata>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_color: Option<String>,
    pub products: Vec<String>,
    pub routing_numbers: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<InstitutionStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
impl std::fmt::Display for Institution {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}