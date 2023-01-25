
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Numbers {
    pub account: Option<String>,
    pub ach_routing: Option<String>,
    pub ach_wire_routing: Option<String>,
    pub bacs_sort_code: Option<String>,
    pub eft_branch: Option<String>,
    pub eft_institution: Option<String>,
    pub international_bic: Option<String>,
    pub international_iban: Option<String>,
}
impl std::fmt::Display for Numbers {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}