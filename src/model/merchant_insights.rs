use serde::{Serialize, Deserialize};
///Insights into a user’s top merchants.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MerchantInsights {
    ///A unique, stable, Plaid-generated id that maps to the merchant.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub entity_id: Option<String>,
    ///The counterparty name.
    pub name: String,
    ///The detailed personal finance category associated with this merchant.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub personal_finance_category_detailed: Option<String>,
    ///The primary personal finance category associated with this merchant.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub personal_finance_category_primary: Option<String>,
    ///Sum of inflow amounts.
    pub total_inflows: f64,
    ///Sum of outflow amounts.
    pub total_outflows: f64,
    ///The number of transactions associated with merchant of this type.
    pub transaction_count: i64,
    ///The website associated with the merchant.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub website: Option<String>,
}
impl std::fmt::Display for MerchantInsights {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}