use serde::{Serialize, Deserialize};
///Defines the response schema for `/bank_transfer/migrate_account`
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BankTransferMigrateAccountResponse {
    ///The Plaid `access_token` for the newly created Item.
    pub access_token: String,
    ///The Plaid `account_id` for the newly created Item.
    pub account_id: String,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
}
impl std::fmt::Display for BankTransferMigrateAccountResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}