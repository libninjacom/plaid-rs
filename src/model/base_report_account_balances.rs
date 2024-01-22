use serde::{Serialize, Deserialize};
///Base Report information about an account's balances
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BaseReportAccountBalances {}
impl std::fmt::Display for BaseReportAccountBalances {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}