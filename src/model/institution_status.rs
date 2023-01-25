
use serde::{Serialize, Deserialize};
use super::{ProductStatus, HealthIncident};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct InstitutionStatus {
    pub auth: Option<ProductStatus>,
    pub health_incidents: Option<Vec<HealthIncident>>,
    pub identity: Option<ProductStatus>,
    pub investments: Option<ProductStatus>,
    pub investments_updates: Option<ProductStatus>,
    pub item_logins: Option<ProductStatus>,
    pub liabilities: Option<ProductStatus>,
    pub liabilities_updates: Option<ProductStatus>,
    pub transactions_updates: Option<ProductStatus>,
}
impl std::fmt::Display for InstitutionStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}