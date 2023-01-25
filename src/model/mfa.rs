
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Mfa {
    pub question_rounds: f64,
    pub questions_per_round: f64,
    pub selection_rounds: f64,
    pub selections_per_question: f64,
    #[serde(rename = "type")]
    pub type_: String,
}
impl std::fmt::Display for Mfa {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}