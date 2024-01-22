use serde::{Serialize, Deserialize};
///The rationale for Plaid's decision to not guarantee a transfer. Will be `null` unless `guarantee_decision` is `NOT_GUARANTEED`.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TransferAuthorizationGuaranteeDecisionRationale {
    /**A code representing the reason Plaid declined to guarantee this transfer:

`RETURN_BANK`: The risk of a bank-initiated return (for example, an R01/NSF) is too high to guarantee this transfer.

`RETURN_CUSTOMER`: The risk of a customer-initiated return (for example, a R10/Unauthorized) is too high to guarantee this transfer.

`GUARANTEE_LIMIT_REACHED`: This transfer is low-risk, but Guarantee has exhausted an internal limit on the number or rate of guarantees that applies to this transfer.

`RISK_ESTIMATE_UNAVAILABLE`: A risk estimate is unavailable for this Item.

`REQUIRED_PARAM_MISSING`: Required fields are missing.*/
    pub code: String,
    ///A human-readable description of why the transfer cannot be guaranteed.
    pub description: String,
}
impl std::fmt::Display for TransferAuthorizationGuaranteeDecisionRationale {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}