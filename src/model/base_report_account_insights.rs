use serde::{Serialize, Deserialize};
use super::{
    BaseReportAverageFlowInsights, BaseReportLongestGapInsights,
    BaseReportNumberFlowInsights,
};
///Calculated insights derived from transaction-level data.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BaseReportAccountInsights {
    ///Average number of days between sequential transactions
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub average_days_between_transactions: Option<f64>,
    ///Average amount of debit transactions into account. This field will be null for non-depository accounts. This field only takes into account USD transactions from the account.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub average_inflow_amount: Option<Vec<BaseReportAverageFlowInsights>>,
    ///Average amount of credit transactions into account. This field will be null for non-depository accounts. This field only takes into account USD transactions from the account.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub average_outflow_amount: Option<Vec<BaseReportAverageFlowInsights>>,
    ///Number of days days available in the base report for the account.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub days_available: Option<i64>,
    ///Longest gap between sequential transactions
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub longest_gap_between_transactions: Option<Vec<BaseReportLongestGapInsights>>,
    ///Date of the most recent transaction in the base report for the account.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub most_recent_transaction_date: Option<chrono::NaiveDate>,
    ///Number of days with no transactions
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub number_of_days_no_transactions: Option<i64>,
    ///The number of debits into the account. This field will be null for non-depository accounts.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub number_of_inflows: Option<Vec<BaseReportNumberFlowInsights>>,
    ///The number of credit into the account. This field will be null for non-depository accounts.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub number_of_outflows: Option<Vec<BaseReportNumberFlowInsights>>,
    ///Date of the earliest transaction in the base report for the account.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub oldest_transaction_date: Option<chrono::NaiveDate>,
}
impl std::fmt::Display for BaseReportAccountInsights {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}