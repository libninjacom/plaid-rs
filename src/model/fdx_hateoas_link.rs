
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FdxHateoasLink {
    pub action: Option<String>,
    pub href: String,
    pub rel: Option<String>,
    pub types: Option<Vec<String>>,
}
impl std::fmt::Display for FdxHateoasLink {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}