use serde::{Serialize, Deserialize};
use super::Role;
///Documentation not found in the MISMO model viewer and not provided by Freddie Mac.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Roles {
    ///ADocumentation not found in the MISMO model viewer and not provided by Freddie Mac.
    #[serde(rename = "ROLE")]
    pub role: Role,
}
impl std::fmt::Display for Roles {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}