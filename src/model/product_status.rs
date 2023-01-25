
use serde::{Serialize, Deserialize};
use super::ProductStatusBreakdown;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductStatus {
    pub breakdown: ProductStatusBreakdown,
    pub last_status_change: String,
    pub status: String,
}
impl std::fmt::Display for ProductStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}