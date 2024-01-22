use serde::{Serialize, Deserialize};
///Data on the W2 Box 12
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct W2Box12 {
    ///W2 Box 12 amount.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub amount: Option<String>,
    ///W2 Box 12 code.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
}
impl std::fmt::Display for W2Box12 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}