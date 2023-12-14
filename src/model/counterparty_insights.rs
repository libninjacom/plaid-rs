
use serde::{Serialize, Deserialize};
use super::{FinancialInstitutionInsights, MerchantInsights};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CounterpartyInsights {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub financial_institution_insights: Option<Vec<FinancialInstitutionInsights>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merchant_insights: Option<Vec<MerchantInsights>>,
}
impl std::fmt::Display for CounterpartyInsights {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}