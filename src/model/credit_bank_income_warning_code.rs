
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Serialize, Deserialize)]
pub enum CreditBankIncomeWarningCode {
    #[serde(rename = "IDENTITY_UNAVAILABLE")]
    IdentityUnavailable,
    #[serde(rename = "TRANSACTIONS_UNAVAILABLE")]
    TransactionsUnavailable,
    #[serde(rename = "ITEM_UNAPPROVED")]
    ItemUnapproved,
    #[serde(rename = "REPORT_DELETED")]
    ReportDeleted,
}