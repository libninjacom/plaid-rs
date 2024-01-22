use serde::{Serialize, Deserialize};
use super::{BankInitiatedReturnRisk, CustomerInitiatedReturnRisk};
///Risk scoring details broken down by risk category.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SignalScores {
    ///The object contains a risk score and a risk tier that evaluate the transaction return risk because an account is overdrawn or because an ineligible account is used. Common return codes in this category include: "R01", "R02", "R03", "R04", "R06", "R08",  "R09", "R13", "R16", "R17", "R20", "R23". These returns have a turnaround time of 2 banking days.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bank_initiated_return_risk: Option<BankInitiatedReturnRisk>,
    ///The object contains a risk score and a risk tier that evaluate the transaction return risk of an unauthorized debit. Common return codes in this category include: "R05", "R07", "R10", "R11", "R29". These returns typically have a return time frame of up to 60 calendar days. During this period, the customer of financial institutions can dispute a transaction as unauthorized.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub customer_initiated_return_risk: Option<CustomerInitiatedReturnRisk>,
}
impl std::fmt::Display for SignalScores {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}