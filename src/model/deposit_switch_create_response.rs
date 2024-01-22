use serde::{Serialize, Deserialize};
///DepositSwitchCreateResponse defines the response schema for `/deposit_switch/create`
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DepositSwitchCreateResponse {
    ///ID of the deposit switch. This ID is persisted throughout the lifetime of the deposit switch.
    pub deposit_switch_id: String,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
}
impl std::fmt::Display for DepositSwitchCreateResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}