use serde::{Serialize, Deserialize};
///An institution object. If the Item was created via Same-Day micro-deposit verification, will be `null`.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LinkSessionSuccessMetadataInstitution {
    ///The Plaid institution identifier
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub institution_id: Option<String>,
    ///The full institution name, such as `'Wells Fargo'`
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl std::fmt::Display for LinkSessionSuccessMetadataInstitution {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}