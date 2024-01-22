use serde::{Serialize, Deserialize};
///A possible account detected to be associated with a transaction user.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DetectedAccount {
    ///The detected subtype of the account, based on the transactions to/from the institution.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub account_subtype: Option<String>,
    ///The detected account type (depository, credit, loan, investment etc.).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub account_type: Option<String>,
    ///Amount of the most recent transaction associated with this detected account type at this financial institution.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub newest_transaction_amount: Option<f64>,
    ///The date of the newest transaction associated with this detected account type at this financial institution.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub newest_transaction_date: Option<chrono::NaiveDate>,
    ///The date of the oldest transaction associated with this detected account type at this financial institution.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub oldest_transaction_date: Option<chrono::NaiveDate>,
    ///Sum of inflow amounts associated with this detected account type at this financial institution.
    pub total_inflows: f64,
    ///Sum of outflow amounts associated with this detected account type at this financial institution.
    pub total_outflows: f64,
    ///The number of transactions associated with this detected account type at this financial institution.
    pub transaction_count: i64,
}
impl std::fmt::Display for DetectedAccount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}