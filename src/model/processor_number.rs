
use serde::{Serialize, Deserialize};
use super::{NumbersAch, NumbersBacs, NumbersEft, NumbersInternational};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ProcessorNumber {
    pub ach: Option<NumbersAch>,
    pub bacs: Option<NumbersBacs>,
    pub eft: Option<NumbersEft>,
    pub international: Option<NumbersInternational>,
}
impl std::fmt::Display for ProcessorNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}