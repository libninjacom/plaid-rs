use serde::{Serialize, Deserialize};
use super::{
    CreditBankStatementUploadAccountOwner, CreditBankStatementUploadBankAccountPeriod,
};
///An object containing data about a user's bank account related to an uploaded bank statement.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreditBankStatementUploadBankAccount {
    ///The unique id of the bank account
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    ///The bank account number.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub account_number: Option<String>,
    ///The type of the bank account.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub account_type: Option<String>,
    ///The name of the bank institution.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bank_name: Option<String>,
    ///The name of the bank account
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    ///An object containing data about the owner of the bank account for the uploaded bank statement.
    pub owner: CreditBankStatementUploadAccountOwner,
    ///An array of period objects, containing more data on the overall period of the statement.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub periods: Vec<CreditBankStatementUploadBankAccountPeriod>,
}
impl std::fmt::Display for CreditBankStatementUploadBankAccount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}