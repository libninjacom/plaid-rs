
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountSubtypes(pub Vec<AccountSubtype>);