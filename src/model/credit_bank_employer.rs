use serde::{Serialize, Deserialize};
///Object containing employer data.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreditBankEmployer {
    ///Name of the employer.
    pub name: String,
}
impl std::fmt::Display for CreditBankEmployer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}