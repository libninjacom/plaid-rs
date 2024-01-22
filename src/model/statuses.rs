use serde::{Serialize, Deserialize};
use super::Status;
///A collection of STATUS containers.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Statuses {
    ///Documentation not found in the MISMO model viewer and not provided by Freddie Mac.
    #[serde(rename = "STATUS")]
    pub status: Status,
}
impl std::fmt::Display for Statuses {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}