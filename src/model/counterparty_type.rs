
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Serialize, Deserialize)]
pub enum CounterpartyType {
    #[serde(rename = "merchant")]
    Merchant,
    #[serde(rename = "financial_institution")]
    FinancialInstitution,
    #[serde(rename = "payment_app")]
    PaymentApp,
    #[serde(rename = "marketplace")]
    Marketplace,
    #[serde(rename = "payment_terminal")]
    PaymentTerminal,
}