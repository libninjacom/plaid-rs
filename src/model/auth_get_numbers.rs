
use serde::{Serialize, Deserialize};
use super::{NumbersAch, NumbersBacs, NumbersEft, NumbersInternational};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AuthGetNumbers {
    pub ach: Vec<NumbersAch>,
    pub bacs: Vec<NumbersBacs>,
    pub eft: Vec<NumbersEft>,
    pub international: Vec<NumbersInternational>,
}
impl std::fmt::Display for AuthGetNumbers {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}