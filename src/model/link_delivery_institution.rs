use serde::{Serialize, Deserialize};
///Information related to the financial institution.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LinkDeliveryInstitution {
    ///The Plaid institution identifier
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub institution_id: Option<String>,
    ///The full institution name, such as 'Wells Fargo'
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl std::fmt::Display for LinkDeliveryInstitution {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}