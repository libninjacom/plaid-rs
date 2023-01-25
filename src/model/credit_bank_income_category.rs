
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Serialize, Deserialize)]
pub enum CreditBankIncomeCategory {
    #[serde(rename = "SALARY")]
    Salary,
    #[serde(rename = "UNEMPLOYMENT")]
    Unemployment,
    #[serde(rename = "CASH")]
    Cash,
    #[serde(rename = "GIG_ECONOMY")]
    GigEconomy,
    #[serde(rename = "RENTAL")]
    Rental,
    #[serde(rename = "CHILD_SUPPORT")]
    ChildSupport,
    #[serde(rename = "MILITARY")]
    Military,
    #[serde(rename = "RETIREMENT")]
    Retirement,
    #[serde(rename = "LONG_TERM_DISABILITY")]
    LongTermDisability,
    #[serde(rename = "BANK_INTEREST")]
    BankInterest,
    #[serde(rename = "CASH_DEPOSIT")]
    CashDeposit,
    #[serde(rename = "TRANSFER_FROM_APPLICATION")]
    TransferFromApplication,
    #[serde(rename = "TAX_REFUND")]
    TaxRefund,
    #[serde(rename = "OTHER")]
    Other,
}