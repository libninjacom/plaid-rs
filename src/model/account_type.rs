
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Serialize, Deserialize)]
pub enum AccountType {
    #[serde(rename = "investment")]
    Investment,
    #[serde(rename = "credit")]
    Credit,
    #[serde(rename = "depository")]
    Depository,
    #[serde(rename = "loan")]
    Loan,
    #[serde(rename = "brokerage")]
    Brokerage,
    #[serde(rename = "other")]
    Other,
}