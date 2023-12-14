
use serde::{Serialize, Deserialize};
use super::RecurringFrequency;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Recurrence {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency: Option<RecurringFrequency>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_recurring: Option<bool>,
}
impl std::fmt::Display for Recurrence {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}