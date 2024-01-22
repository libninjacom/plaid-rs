use serde::{Serialize, Deserialize};
///Fired when the status of a deposit switch request has changed.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DepositSwitchStateUpdateWebhook {
    ///The ID of the deposit switch.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deposit_switch_id: Option<String>,
    ///The Plaid environment the webhook was sent from
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub environment: Option<String>,
    /**The state, or status, of the deposit switch.

`initialized`: The deposit switch has been initialized with the user entering the information required to submit the deposit switch request.

`processing`: The deposit switch request has been submitted and is being processed.

`completed`: The user's employer has fulfilled and completed the deposit switch request.

`error`: There was an error processing the deposit switch request.

For more information, see the [Deposit Switch API reference](/docs/deposit-switch/reference#deposit_switchget).*/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    ///`"SWITCH_STATE_UPDATE"`
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub webhook_code: Option<String>,
    ///`"DEPOSIT_SWITCH"`
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub webhook_type: Option<String>,
}
impl std::fmt::Display for DepositSwitchStateUpdateWebhook {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}