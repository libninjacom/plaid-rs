
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmploymentVerificationStatus(pub serde_json::Value);