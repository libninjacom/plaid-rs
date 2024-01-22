use serde::{Serialize, Deserialize};
///IncomeVerificationPrecheckResponse defines the response schema for `/income/verification/precheck`.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct IncomeVerificationPrecheckResponse {
    /**The confidence that Plaid can support the user in the digital income verification flow instead of requiring a manual paystub upload. One of the following:

`"HIGH"`: It is very likely that this user can use the digital income verification flow.

"`LOW`": It is unlikely that this user can use the digital income verification flow.

`"UNKNOWN"`: It was not possible to determine if the user is supportable with the information passed.*/
    pub confidence: String,
    ///ID of the precheck. Provide this value when calling `/link/token/create` in order to optimize Link conversion.
    pub precheck_id: String,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
}
impl std::fmt::Display for IncomeVerificationPrecheckResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}