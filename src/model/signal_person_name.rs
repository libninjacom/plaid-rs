use serde::{Serialize, Deserialize};
///The user's legal name
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SignalPersonName {
    ///The user's family name / surname
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub family_name: Option<String>,
    ///The user's given name. If the user has a one-word name, it should be provided in this field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub given_name: Option<String>,
    ///The user's middle name
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub middle_name: Option<String>,
    ///The user's name prefix (e.g. "Mr.")
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
    ///The user's name suffix (e.g. "II")
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub suffix: Option<String>,
}
impl std::fmt::Display for SignalPersonName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}