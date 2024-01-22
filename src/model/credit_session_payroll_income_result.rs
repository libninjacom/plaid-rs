use serde::{Serialize, Deserialize};
///The details of a digital payroll income verification in Link
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreditSessionPayrollIncomeResult {
    ///The Plaid Institution ID associated with the Item.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub institution_id: Option<String>,
    ///The Institution Name associated with the Item.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub institution_name: Option<String>,
    ///The number of paystubs retrieved from a payroll provider.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub num_paystubs_retrieved: Option<i64>,
    ///The number of w2s retrieved from a payroll provider.
    #[serde(rename = "num_w2s_retrieved")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub num_w2_s_retrieved: Option<i64>,
}
impl std::fmt::Display for CreditSessionPayrollIncomeResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}