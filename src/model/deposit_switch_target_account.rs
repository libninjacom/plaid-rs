use serde::{Serialize, Deserialize};
///The deposit switch destination account
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DepositSwitchTargetAccount {
    ///The name of the deposit switch destination account, as it will be displayed to the end user in the Deposit Switch interface. It is not required to match the name used in online banking.
    pub account_name: String,
    ///Account number for deposit switch destination
    pub account_number: String,
    ///The account subtype of the account, either `checking` or `savings`.
    pub account_subtype: String,
    ///Routing number for deposit switch destination
    pub routing_number: String,
}
impl std::fmt::Display for DepositSwitchTargetAccount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}