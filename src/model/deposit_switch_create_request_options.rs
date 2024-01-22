use serde::{Serialize, Deserialize};
///Options to configure the `/deposit_switch/create` request. If provided, cannot be `null`.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DepositSwitchCreateRequestOptions {
    ///An array of access tokens corresponding to transaction items to use when attempting to match the user to their Payroll Provider. These tokens must be created by the same client id as the one creating the switch, and have access to the transactions product.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transaction_item_access_tokens: Option<Vec<String>>,
    ///The URL registered to receive webhooks when the status of a deposit switch request has changed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub webhook: Option<String>,
}
impl std::fmt::Display for DepositSwitchCreateRequestOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}