use serde::{Serialize, Deserialize};
///Parent container for name that allows for choice group between parsed and unparsed containers.Parent container for name that allows for choice group between parsed and unparsed containers.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct IndividualName {
    ///The first name of the individual represented by the parent object.
    #[serde(rename = "FirstName")]
    pub first_name: String,
    ///The last name of the individual represented by the parent object.
    #[serde(rename = "LastName")]
    pub last_name: String,
}
impl std::fmt::Display for IndividualName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}