use serde::{Serialize, Deserialize};
///An object containing employer data.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EmployerVerification {
    ///Name of employer.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl std::fmt::Display for EmployerVerification {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}