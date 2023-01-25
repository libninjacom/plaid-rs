
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Serialize, Deserialize)]
pub enum CreditAccountSubtype {
    #[serde(rename = "credit card")]
    CreditCard,
    #[serde(rename = "paypal")]
    Paypal,
    #[serde(rename = "all")]
    All,
}