use serde::{Serialize, Deserialize};
///The object contains a risk score and a risk tier that evaluate the transaction return risk of an unauthorized debit. Common return codes in this category include: "R05", "R07", "R10", "R11", "R29". These returns typically have a return time frame of up to 60 calendar days. During this period, the customer of financial institutions can dispute a transaction as unauthorized.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CustomerInitiatedReturnRisk {
    /**A tier corresponding to the projected likelihood that the transaction, if initiated, will be subject to a return.

In the `customer_initiated_return_risk` object, there are five risk tiers corresponding to the scores:
  1: Predicted customer-initiated return incidence rate between 0.00% - 0.02%
  2: Predicted customer-initiated return incidence rate between 0.02% - 0.05%
  3: Predicted customer-initiated return incidence rate between 0.05% - 0.1%
  4: Predicted customer-initiated return incidence rate between 0.1% - 0.5%
  5: Predicted customer-initiated return incidence rate greater than 0.5%*/
    pub risk_tier: i64,
    ///A score from 1-99 that indicates the transaction return risk: a higher risk score suggests a higher return likelihood.
    pub score: i64,
}
impl std::fmt::Display for CustomerInitiatedReturnRisk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}