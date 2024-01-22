use serde::{Serialize, Deserialize};
///Documentation not found in the MISMO model viewer and not provided by Freddie Mac.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Status {
    ///Satus Code.
    #[serde(rename = "StatusCode")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status_code: Option<String>,
    ///Status Description.
    #[serde(rename = "StatusDescription")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status_description: Option<String>,
}
impl std::fmt::Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}