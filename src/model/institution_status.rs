
use serde::{Serialize, Deserialize};
use super::{HealthIncident, ProductStatus};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct InstitutionStatus {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth: Option<ProductStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_incidents: Option<Vec<HealthIncident>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity: Option<ProductStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub investments: Option<ProductStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub investments_updates: Option<ProductStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_logins: Option<ProductStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub liabilities: Option<ProductStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub liabilities_updates: Option<ProductStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transactions_updates: Option<ProductStatus>,
}
impl std::fmt::Display for InstitutionStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}