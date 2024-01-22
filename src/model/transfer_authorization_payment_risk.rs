use serde::{Serialize, Deserialize};
use super::SignalWarning;
///This object includes the scores and risk level. This response is offered as an add-on to /transfer/authorization/create. To request access to these fields please contact your Plaid account manager.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TransferAuthorizationPaymentRisk {
    /**A score from 1-99 that indicates the transaction return risk: a higher risk score suggests a higher return likelihood.

The score evaluates the transaction return risk because an account is overdrawn or because an ineligible account is used and covers return codes: "R01", "R02", "R03", "R04", "R06", "R08",  "R09", "R13",
"R16", "R17", "R20", "R23". These returns have a turnaround time of 2 banking days.*/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bank_initiated_return_score: Option<i64>,
    /**A score from 1-99 that indicates the transaction return risk: a higher risk score suggests a higher return likelihood.

The score evaluates the transaction return risk of an unauthorized debit and covers return codes: "R05", "R07", "R10", "R11", "R29".
These returns typically have a return time frame of up to 60 calendar days. During this period, the customer of financial institutions can dispute a transaction as unauthorized.*/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub customer_initiated_return_score: Option<i64>,
    ///Comprises five risk categories (high risk, medium-high risk, medium risk, medium-low risk, low risk) based on the probability of return
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub risk_level: Option<String>,
    ///If bank information was not available to be used in the Signal model, this array contains warnings describing why bank data is missing. If you want to receive an API error instead of Signal scores in the case of missing bank data, file a support ticket or contact your Plaid account manager.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub warnings: Vec<SignalWarning>,
}
impl std::fmt::Display for TransferAuthorizationPaymentRisk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}