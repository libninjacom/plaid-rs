
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BankTransferEventListBankTransferType(pub serde_json::Value);