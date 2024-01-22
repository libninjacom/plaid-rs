use serde::{Serialize, Deserialize};
///Assets under management for the given end customer. Required for end customers with monthly service commitments.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PartnerEndCustomerAssetsUnderManagement {
    pub amount: f64,
    pub iso_currency_code: String,
}
impl std::fmt::Display for PartnerEndCustomerAssetsUnderManagement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}