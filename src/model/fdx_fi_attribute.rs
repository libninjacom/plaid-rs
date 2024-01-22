use serde::{Serialize, Deserialize};
///Financial Institution provider-specific attribute
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FdxFiAttribute {
    ///Name of attribute
    pub name: String,
    ///Value of attribute
    pub value: String,
}
impl std::fmt::Display for FdxFiAttribute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}