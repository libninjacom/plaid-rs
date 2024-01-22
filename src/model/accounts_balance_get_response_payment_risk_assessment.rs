use serde::{Serialize, Deserialize};
///This object provides detailed risk assessment for the requested transaction
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AccountsBalanceGetResponsePaymentRiskAssessment {
    ///Indicates whether the requested ACH debit amount is greater than the threshold (set by customers) of the available or current balance.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exceeds_balance_threshold: Option<bool>,
    ///A five-tier risk assessment for the transaction based on the probability of return.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub risk_level: Option<String>,
}
impl std::fmt::Display for AccountsBalanceGetResponsePaymentRiskAssessment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}