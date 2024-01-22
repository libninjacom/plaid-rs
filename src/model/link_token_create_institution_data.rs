use serde::{Serialize, Deserialize};
///A map containing data used to highlight institutions in Link.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LinkTokenCreateInstitutionData {
    ///The routing number of the bank to highlight in Link. Note: in rare cases, a single routing number can be associated with multiple institutions, e.g. due to a brokerage using another institution to manage ACH on its sweep accounts. If this happens, the bank will not be highlighted in Link even if the routing number is provided.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub routing_number: Option<String>,
}
impl std::fmt::Display for LinkTokenCreateInstitutionData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}