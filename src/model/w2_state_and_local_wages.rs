use serde::{Serialize, Deserialize};
///W2 state and local wages
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct W2StateAndLocalWages {
    ///State identification number of the employer.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub employer_state_id_number: Option<String>,
    ///Income tax from the locality.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub local_income_tax: Option<String>,
    ///Wages and tips from the locality.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub local_wages_tips: Option<String>,
    ///Name of the locality.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub locality_name: Option<String>,
    ///State associated with the wage.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    ///Income tax from the specified state.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state_income_tax: Option<String>,
    ///Wages and tips from the specified state.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state_wages_tips: Option<String>,
}
impl std::fmt::Display for W2StateAndLocalWages {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}