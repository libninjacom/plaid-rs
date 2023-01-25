
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessorNumber {
    pub ach: NumbersAchNullable,
    pub bacs: NumbersBacsNullable,
    pub eft: NumbersEftNullable,
    pub international: NumbersInternationalNullable,
}
impl std::fmt::Display for ProcessorNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}