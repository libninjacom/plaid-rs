use serde::{Serialize, Deserialize};
///Documentation not found in the MISMO model viewer and not provided by Freddie Mac.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreditFreddieMacIndividualName {
    ///The first name of the individual represented by the parent object.
    #[serde(rename = "FirstName")]
    pub first_name: String,
    ///The last name of the individual represented by the parent object.
    #[serde(rename = "LastName")]
    pub last_name: String,
    ///The middle name of the individual represented by the parent object.
    #[serde(rename = "MiddleName")]
    pub middle_name: String,
}
impl std::fmt::Display for CreditFreddieMacIndividualName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}