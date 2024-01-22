use serde::{Serialize, Deserialize};
use super::{FinancialInstitutionInsights, MerchantInsights};
///Insights around a user's counterparties
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CounterpartyInsights {
    ///Insights related to a user’s transactions with other financial institutions, including detected account types.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub financial_institution_insights: Option<Vec<FinancialInstitutionInsights>>,
    ///Insights about a user’s top merchants, ranked by spend.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub merchant_insights: Option<Vec<MerchantInsights>>,
}
impl std::fmt::Display for CounterpartyInsights {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}