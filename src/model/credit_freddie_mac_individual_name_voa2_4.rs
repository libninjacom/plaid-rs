
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreditFreddieMacIndividualNameVoa24 {
    #[serde(rename = "FirstName")]
    pub first_name: String,
    #[serde(rename = "LastName")]
    pub last_name: String,
    #[serde(rename = "MiddleName")]
    pub middle_name: String,
}
impl std::fmt::Display for CreditFreddieMacIndividualNameVoa24 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}