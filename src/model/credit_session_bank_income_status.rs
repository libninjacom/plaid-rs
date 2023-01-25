
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Serialize, Deserialize)]
pub enum CreditSessionBankIncomeStatus {
    #[serde(rename = "APPROVED")]
    Approved,
    #[serde(rename = "NO_DEPOSITS_FOUND")]
    NoDepositsFound,
    #[serde(rename = "USER_REPORTED_NO_INCOME")]
    UserReportedNoIncome,
}