
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Serialize, Deserialize)]
pub enum AssetType {
    CheckingAccount,
    SavingsAccount,
    Investment,
    MoneyMarketFund,
    Other,
}