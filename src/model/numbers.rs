
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Numbers {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ach_routing: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ach_wire_routing: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bacs_sort_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eft_branch: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eft_institution: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub international_bic: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub international_iban: Option<String>,
}
impl std::fmt::Display for Numbers {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}