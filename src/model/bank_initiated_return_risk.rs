use serde::{Serialize, Deserialize};
///The object contains a risk score and a risk tier that evaluate the transaction return risk because an account is overdrawn or because an ineligible account is used. Common return codes in this category include: "R01", "R02", "R03", "R04", "R06", "R08",  "R09", "R13", "R16", "R17", "R20", "R23". These returns have a turnaround time of 2 banking days.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BankInitiatedReturnRisk {
    /**In the `bank_initiated_return_risk` object, there are eight risk tiers corresponding to the scores:
  1: Predicted bank-initiated return incidence rate between 0.0% - 0.5%
  2: Predicted bank-initiated return incidence rate between 0.5% - 1.5%
  3: Predicted bank-initiated return incidence rate between 1.5% - 3%
  4: Predicted bank-initiated return incidence rate between 3% - 5%
  5: Predicted bank-initiated return incidence rate between 5% - 10%
  6: Predicted bank-initiated return incidence rate between 10% - 15%
  7: Predicted bank-initiated return incidence rate between 15% and 50%
  8: Predicted bank-initiated return incidence rate greater than 50%*/
    pub risk_tier: i64,
    ///A score from 1-99 that indicates the transaction return risk: a higher risk score suggests a higher return likelihood.
    pub score: i64,
}
impl std::fmt::Display for BankInitiatedReturnRisk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}