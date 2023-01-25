
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Serialize, Deserialize)]
pub enum PaymentAmountCurrency {
    #[serde(rename = "GBP")]
    Gbp,
    #[serde(rename = "EUR")]
    Eur,
    #[serde(rename = "PLN")]
    Pln,
    #[serde(rename = "SEK")]
    Sek,
    #[serde(rename = "DKK")]
    Dkk,
    #[serde(rename = "NOK")]
    Nok,
}