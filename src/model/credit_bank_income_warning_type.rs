
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Serialize, Deserialize)]
pub enum CreditBankIncomeWarningType {
    #[serde(rename = "BANK_INCOME_WARNING")]
    BankIncomeWarning,
}