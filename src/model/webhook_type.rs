
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Serialize, Deserialize)]
pub enum WebhookType {
    #[serde(rename = "AUTH")]
    Auth,
    #[serde(rename = "HOLDINGS")]
    Holdings,
    #[serde(rename = "INVESTMENTS_TRANSACTIONS")]
    InvestmentsTransactions,
    #[serde(rename = "ITEM")]
    Item,
    #[serde(rename = "LIABILITIES")]
    Liabilities,
    #[serde(rename = "TRANSACTIONS")]
    Transactions,
}