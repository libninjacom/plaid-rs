use serde::{Serialize, Deserialize};
use super::{AccountBase, AccountsBalanceGetResponsePaymentRiskAssessment, Item};
///AccountsGetResponse defines the response schema for `/accounts/get` and `/accounts/balance/get`.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AccountsGetResponse {
    /**An array of financial institution accounts associated with the Item.
If `/accounts/balance/get` was called, each account will include real-time balance information.*/
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub accounts: Vec<AccountBase>,
    ///Metadata about the Item.
    pub item: Item,
    ///This object provides detailed risk assessment for the requested transaction
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub payment_risk_assessment: Option<AccountsBalanceGetResponsePaymentRiskAssessment>,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
}
impl std::fmt::Display for AccountsGetResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}