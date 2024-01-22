use serde::{Serialize, Deserialize};
///Result summary object specifying values for `phone` attributes of risk check.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RiskCheckPhone {
    ///A list of online services where this phone number has been detected to have accounts or other activity.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub linked_services: Vec<String>,
}
impl std::fmt::Display for RiskCheckPhone {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}