use serde::{Serialize, Deserialize};
///The originator's funding account, linked with Plaid Link or `/transfer/migrate_account`.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TransferFundingAccount {
    ///The access token associated with the Item data is being requested for.
    pub access_token: String,
    ///The Plaid `account_id` for the newly created Item.
    pub account_id: String,
}
impl std::fmt::Display for TransferFundingAccount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}