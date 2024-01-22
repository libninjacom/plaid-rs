use serde::{Serialize, Deserialize};
///Originator and their status.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Originator {
    ///The company name of the end customer.
    pub company_name: String,
    ///Originator’s diligence status.
    pub transfer_diligence_status: String,
}
impl std::fmt::Display for Originator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}