
use serde::{Serialize, Deserialize};
use super::{NumbersAch, NumbersBacs, NumbersEft, NumbersInternational};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ProcessorNumber {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ach: Option<NumbersAch>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bacs: Option<NumbersBacs>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eft: Option<NumbersEft>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub international: Option<NumbersInternational>,
}
impl std::fmt::Display for ProcessorNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}