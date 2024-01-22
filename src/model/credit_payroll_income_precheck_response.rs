use serde::{Serialize, Deserialize};
///Defines the response schema for `/credit/payroll_income/precheck`.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreditPayrollIncomePrecheckResponse {
    /**The confidence that Plaid can support the user in the digital income verification flow instead of requiring a manual paystub upload. One of the following:

`"HIGH"`: It is very likely that this user can use the digital income verification flow.

"`LOW`": It is unlikely that this user can use the digital income verification flow.

`"UNKNOWN"`: It was not possible to determine if the user is supportable with the information passed.*/
    pub confidence: String,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
}
impl std::fmt::Display for CreditPayrollIncomePrecheckResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}