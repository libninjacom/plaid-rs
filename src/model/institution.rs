
use serde::{Serialize, Deserialize};
use super::{AuthMetadata, InstitutionStatus, PaymentInitiationMetadata};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Institution {
    pub auth_metadata: Option<AuthMetadata>,
    pub country_codes: Vec<String>,
    pub institution_id: String,
    pub logo: Option<String>,
    pub name: String,
    pub oauth: bool,
    pub payment_initiation_metadata: Option<PaymentInitiationMetadata>,
    pub primary_color: Option<String>,
    pub products: Vec<String>,
    pub routing_numbers: Vec<String>,
    pub status: Option<InstitutionStatus>,
    pub url: Option<String>,
}
impl std::fmt::Display for Institution {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}