use serde::{Serialize, Deserialize};
///Documentation not found in the MISMO model viewer and not provided by Freddie Mac.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AssetHolderName {
    ///The unparsed name of either an individual or a legal entity.
    #[serde(rename = "FullName")]
    pub full_name: String,
}
impl std::fmt::Display for AssetHolderName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}