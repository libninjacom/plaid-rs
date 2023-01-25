
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Serialize, Deserialize)]
pub enum DepositoryAccountSubtype {
    #[serde(rename = "checking")]
    Checking,
    #[serde(rename = "savings")]
    Savings,
    #[serde(rename = "hsa")]
    Hsa,
    #[serde(rename = "cd")]
    Cd,
    #[serde(rename = "money market")]
    MoneyMarket,
    #[serde(rename = "paypal")]
    Paypal,
    #[serde(rename = "prepaid")]
    Prepaid,
    #[serde(rename = "cash management")]
    CashManagement,
    #[serde(rename = "ebt")]
    Ebt,
    #[serde(rename = "all")]
    All,
}