
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IncomeSummary {
    pub employee_name: EmployeeIncomeSummaryFieldString,
    pub employer_name: EmployerIncomeSummaryFieldString,
    pub pay_frequency: Option<PayFrequency>,
    pub projected_wage: ProjectedIncomeSummaryFieldNumber,
    pub verified_transaction: Option<TransactionData>,
    pub ytd_gross_income: YtdGrossIncomeSummaryFieldNumber,
    pub ytd_net_income: YtdNetIncomeSummaryFieldNumber,
}
impl std::fmt::Display for IncomeSummary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}