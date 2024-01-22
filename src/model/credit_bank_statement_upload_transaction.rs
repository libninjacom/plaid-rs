use serde::{Serialize, Deserialize};
///An object containing data about a transaction appearing on a user-uploaded bank statement.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreditBankStatementUploadTransaction {
    ///The unique id of the bank account that this transaction occurs in
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    ///The value of the transaction. A negative amount indicates that money moved into the account (such as a paycheck being deposited).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub amount: Option<f64>,
    ///The date of when the transaction was made, in ISO 8601 format (YYYY-MM-DD).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub date: Option<chrono::NaiveDate>,
    ///The raw description of the transaction as it appears on the bank statement.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub original_description: Option<String>,
}
impl std::fmt::Display for CreditBankStatementUploadTransaction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}