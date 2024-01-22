use serde::{Serialize, Deserialize};
use super::Asset;
///Documentation not found in the MISMO model viewer and not provided by Freddie Mac.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Assets {
    ///Documentation not found in the MISMO model viewer and not provided by Freddie Mac.
    #[serde(rename = "ASSET")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub asset: Vec<Asset>,
}
impl std::fmt::Display for Assets {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}